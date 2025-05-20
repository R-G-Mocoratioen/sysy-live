use crate::arrayinit::*;
use crate::ast::*;
use crate::constint::*;
use crate::ident;
use crate::ident::*;
use crate::whilecontext::*;
use koopa::ir::builder_traits::*;
use koopa::ir::*;
use std::collections::HashMap;

impl CompUnit {
    fn adddecl(
        &self,
        program: &mut Program,
        var: &mut HashMap<String, IdentValue>,
        id: String,
        rettype: Type,
        paramtype: Vec<Type>,
    ) {
        let func = program.new_func(FunctionData::new(
            ("@".to_owned() + &id).into(),
            paramtype,
            rettype,
        ));
        var.insert(id.clone(), IdentValue::Func(func));
    }

    /*
    decl @getint(): i32
    decl @getch(): i32
    decl @getarray(*i32): i32
    decl @putint(i32)
    decl @putch(i32)
    decl @putarray(i32, *i32)
    decl @starttime()
    decl @stoptime()
    */
    fn gen_libfuncs(&self, program: &mut Program, var: &mut HashMap<String, IdentValue>) {
        self.adddecl(program, var, "getint".into(), Type::get_i32(), Vec::new());
        self.adddecl(program, var, "getch".into(), Type::get_i32(), Vec::new());
        self.adddecl(
            program,
            var,
            "getarray".into(),
            Type::get_i32(),
            vec![Type::get_pointer(Type::get_i32())],
        );
        self.adddecl(
            program,
            var,
            "putint".into(),
            Type::get_unit(),
            vec![Type::get_i32()],
        );
        self.adddecl(
            program,
            var,
            "putch".into(),
            Type::get_unit(),
            vec![Type::get_i32()],
        );
        self.adddecl(
            program,
            var,
            "putarray".into(),
            Type::get_unit(),
            vec![Type::get_i32(), Type::get_pointer(Type::get_i32())],
        );
        self.adddecl(
            program,
            var,
            "starttime".into(),
            Type::get_unit(),
            Vec::new(),
        );
        self.adddecl(
            program,
            var,
            "stoptime".into(),
            Type::get_unit(),
            Vec::new(),
        );
    }

    pub fn gen_ir(&self) -> Program {
        let mut program = Program::new();
        let mut var: HashMap<String, IdentValue> = HashMap::new();
        let mut tmpmap: HashMap<String, i32> = HashMap::new();
        let mut initmap: HashMap<String, Vec<i32>> = HashMap::new();
        let mut sizemap: HashMap<String, Vec<i32>> = HashMap::new();
        {
            // 只能定义一个伪函数
            let pseudo = program.new_func(FunctionData::new(
                "@__pseudo__please_dont_give_same_name".into(),
                Vec::new(),
                Type::get_unit(),
            ));
            let pseudo_data = program.func_mut(pseudo);
            let mut entry = pseudo_data.dfg_mut().new_bb().basic_block(None);
            let _ = pseudo_data.layout_mut().bbs_mut().push_key_back(entry);
            for decl in self.global_var_defs.iter() {
                for vardef in decl.defs.iter() {
                    match vardef.as_ref() {
                        VarDef::Ident(id) => {
                            tmpmap.insert(id.clone(), 0);
                        }
                        VarDef::IdentInit(id, exp) => {
                            let val = exp.gen_ir(pseudo_data, &mut entry, &mut var);
                            if let Some(res) = get_const_int(pseudo_data, val) {
                                tmpmap.insert(id.clone(), res);
                            } else {
                                panic!("global initial value not a constant");
                            }
                        }
                        VarDef::ConstIdentInit(id, exp) => {
                            let val = exp.gen_ir(pseudo_data, &mut entry, &mut var);
                            if let Some(res) = get_const_int(pseudo_data, val) {
                                tmpmap.insert(id.clone(), res.clone());
                                var.insert(id.clone(), IdentValue::ConstValue(res));
                            } else {
                                panic!("global initial value not a constant");
                            }
                        }
                        // 对于数组，init value 和 size 都是常数，都需要借助 pseudo 函数来计算
                        // size 用 sizemap 寸
                        // initvalue 用 initmap 存
                        VarDef::Array(id, exps) => {
                            let mut lens: Vec<i32> = Vec::new();
                            for exp in exps.iter() {
                                let val = exp.gen_ir(pseudo_data, &mut entry, &mut var);
                                if let Some(res) = get_const_int(pseudo_data, val) {
                                    lens.push(res);
                                } else {
                                    panic!("global array size not a constant");
                                }
                            }
                            sizemap.insert(id.clone(), lens.clone());
                        }
                        VarDef::ArrayInit(id, exps, arrayinit) => {
                            let mut lens: Vec<i32> = Vec::new();
                            for exp in exps.iter() {
                                let val = exp.gen_ir(pseudo_data, &mut entry, &mut var);
                                if let Some(res) = get_const_int(pseudo_data, val) {
                                    lens.push(res);
                                } else {
                                    panic!("global array size not a constant");
                                }
                            }
                            sizemap.insert(id.clone(), lens.clone());
                            let res = gen_globalarrayinit(
                                pseudo_data,
                                &mut entry,
                                &mut var,
                                lens,
                                arrayinit,
                            );
                            initmap.insert(id.clone(), res);
                        }
                    }
                }
            }
            let ret = pseudo_data.dfg_mut().new_value().ret(None);
            pseudo_data
                .layout_mut()
                .bb_mut(entry)
                .insts_mut()
                .extend([ret]);
        }

        println!("successfully found the real values of all const global vars");

        {
            // 把值 alloc 进去
            for decl in self.global_var_defs.iter() {
                for vardef in decl.defs.iter() {
                    match vardef.as_ref() {
                        VarDef::Ident(id) => {
                            let me = program.new_value().integer(tmpmap.get(id).unwrap().clone());
                            let alloc = program.new_value().global_alloc(me);
                            var.insert(id.clone(), IdentValue::Value(alloc));
                        }
                        VarDef::IdentInit(id, _) => {
                            let me = program.new_value().integer(tmpmap.get(id).unwrap().clone());
                            let alloc = program.new_value().global_alloc(me);
                            var.insert(id.clone(), IdentValue::Value(alloc));
                        }
                        VarDef::ConstIdentInit(_, _) => {}
                        VarDef::Array(id, _) => {
                            let len = sizemap.get(id).unwrap().clone();
                            let typ = gen_type(len.clone());
                            let alloc = program
                                .new_value()
                                .global_alloc(program.new_value().zero_init(typ));
                            var.insert(id.clone(), IdentValue::Value(alloc));
                        }
                        VarDef::ArrayInit(id, _, _) => {
                            let len = sizemap.get(id).unwrap().clone();
                            let initv = gen_globalinitvalue(
                                &mut program,
                                len.clone(),
                                initmap.get(id).unwrap().clone(),
                            );
                            let alloc = program.new_value().global_alloc(initv);
                            var.insert(id.clone(), IdentValue::Value(alloc));
                        }
                    }
                }
            }
        }

        println!("successfully built global vars");

        self.gen_libfuncs(&mut program, &mut var);

        for func in self.func_defs.iter() {
            let mut typevec = Vec::new();

            for _ in func.params.iter() {
                typevec.push(Type::get_i32()); // 目前只有 int 一个类型
            }

            let main = program.new_func(FunctionData::new(
                ("@".to_owned() + &func.id).into(),
                typevec,
                match func.func_type {
                    FuncType::Int => Type::get_i32(),
                    FuncType::Void => Type::get_unit(),
                },
            ));
            var.insert(func.id.clone(), IdentValue::Func(main));
        }

        for func in self.func_defs.iter() {
            let main = var.get(&func.id).unwrap().clone();
            if let IdentValue::Func(main) = main {
                func.gen_ir(main, &mut program, &mut var);
            }
        }

        program
    }
}

impl FuncDef {
    fn gen_ir(&self, main: Function, program: &mut Program, var: &mut HashMap<String, IdentValue>) {
        let main_data = program.func_mut(main);
        // let mut var: HashMap<String, IdentValue> = HashMap::new();

        let mut entry = main_data.dfg_mut().new_bb().basic_block(None);
        let _ = main_data.layout_mut().bbs_mut().push_key_back(entry);

        let mut myvar = var.clone();

        let mut id = 0;
        for param in self.params.iter() {
            let alloc = main_data.dfg_mut().new_value().alloc(Type::get_i32());
            let funcparamval = main_data.params()[id];
            let load = main_data.dfg_mut().new_value().store(funcparamval, alloc);
            main_data
                .layout_mut()
                .bb_mut(entry)
                .insts_mut()
                .extend([alloc, load]);
            myvar.insert(param.id.clone(), IdentValue::Value(alloc));
            id += 1;
        }

        self.block.gen_ir(main_data, &mut entry, &mut myvar, None);

        // add an unreachable "return" at the end
        let zero = main_data.dfg_mut().new_value().integer(0);
        let ret = main_data.dfg_mut().new_value().ret(match self.func_type {
            FuncType::Int => Some(zero),
            FuncType::Void => None,
        });
        main_data
            .layout_mut()
            .bb_mut(entry)
            .insts_mut()
            .extend([ret]);
    }
}

impl ArrayInit {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
        alloc: Value,
        lens: Vec<i32>,
    ) {
        let mut curpos = 0;
        match self {
            ArrayInit::Single(exp) => panic!("using an exp to init an array"),
            ArrayInit::Multiple(inits) => {
                for cur in inits.iter() {
                    match cur {
                        ArrayInit::Single(exp) => {
                            let val = exp.gen_ir(data, entry, var);
                            let at = gen_arrayelem_ptr(data, entry, alloc, curpos, lens.clone());
                            let store = data.dfg_mut().new_value().store(val, at);
                            data.layout_mut().bb_mut(*entry).insts_mut().extend([store]);
                            curpos += 1;
                        }
                        ArrayInit::Multiple(inits) => {
                            if curpos % lens.last().unwrap() != 0 {
                                panic!("bad array initializer");
                            }
                            let respr = find_firstok(curpos, lens.clone());
                            let firstok = respr.0;
                            let curcur = respr.1;
                            // 用 inits 去匹配 firstok 之后的
                            let at = gen_arrayelem_ptr(
                                data,
                                entry,
                                alloc,
                                curpos / curcur,
                                lens.clone()[0..firstok].to_vec(),
                            );
                            inits.gen_ir(data, entry, var, at, lens.clone()[firstok..].to_vec());
                            curpos += lens.clone()[firstok..]
                                .to_vec()
                                .iter()
                                .fold(1, |acc, x| acc * x);
                        }
                    }
                }
                let all = lens.iter().fold(1, |acc, x| acc * x);
                while curpos < all {
                    let val = data.dfg_mut().new_value().integer(0);
                    let at = gen_arrayelem_ptr(data, entry, alloc, curpos, lens.clone());
                    let store = data.dfg_mut().new_value().store(val, at);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([store]);
                    curpos += 1;
                }
            }
        }
    }
}

impl VarDef {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) {
        match self {
            VarDef::Ident(id) => {
                let alloc = data.dfg_mut().new_value().alloc(Type::get_i32());
                data.layout_mut().bb_mut(*entry).insts_mut().extend([alloc]);
                var.insert(id.clone(), IdentValue::Value(alloc));
            }
            VarDef::IdentInit(id, exp) => {
                let alloc = data.dfg_mut().new_value().alloc(Type::get_i32());
                data.layout_mut().bb_mut(*entry).insts_mut().extend([alloc]);
                var.insert(id.clone(), IdentValue::Value(alloc));
                let val = exp.gen_ir(data, entry, var);
                let store = data.dfg_mut().new_value().store(val, alloc);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([store]);
            }
            VarDef::ConstIdentInit(id, exp) => {
                let val = exp.gen_ir(data, entry, var);
                assert!(data.dfg_mut().value(val).kind().is_const());
                var.insert(
                    id.clone(),
                    IdentValue::ConstValue(get_const_int(data, val).unwrap().clone()),
                ); // const 变量直接存值
            }
            VarDef::Array(id, exps) => {
                let mut lens: Vec<i32> = Vec::new();
                for exp in exps.iter() {
                    let val = exp.gen_ir(data, entry, var);
                    if let Some(rv) = get_const_int(data, val) {
                        lens.push(rv);
                    } else {
                        panic!("array size of array {} is not a constant", id);
                    }
                }
                let mut curtype = gen_arraytype(lens);
                let alloc = data.dfg_mut().new_value().alloc(curtype);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([alloc]);
                var.insert(id.clone(), IdentValue::Value(alloc));
            }
            VarDef::ArrayInit(id, exps, arrayinit) => {
                let mut lens: Vec<i32> = Vec::new();
                for exp in exps.iter() {
                    let val = exp.gen_ir(data, entry, var);
                    if let Some(rv) = get_const_int(data, val) {
                        lens.push(rv);
                    } else {
                        panic!("array size of array {} is not a constant", id);
                    }
                }
                let mut curtype = Type::get_i32();
                for dim in lens.iter().rev() {
                    curtype = Type::get_array(curtype, dim as usize);
                }
                let alloc = data.dfg_mut().new_value().alloc(curtype);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([alloc]);
                var.insert(id.clone(), IdentValue::Value(alloc));
                arrayinit.gen_ir(data, entry, var, alloc, lens.clone());
            }
        }
    }
}

impl Decl {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) {
        for item in self.defs.iter() {
            item.gen_ir(data, entry, var);
        }
    }
}

impl Block {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
        lastwhile: Option<WhileContext>,
    ) {
        let mut myvar: HashMap<String, IdentValue> = var.clone();
        for item in &self.vecitem {
            item.gen_ir(data, entry, &mut myvar, lastwhile.clone());
        }
    }
}

impl BlockItem {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
        lastwhile: Option<WhileContext>,
    ) {
        match self {
            BlockItem::Stmt(stmt) => stmt.gen_ir(data, entry, var, lastwhile),
            BlockItem::Decl(decl) => decl.gen_ir(data, entry, var),
        }
    }
}

impl Stmt {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
        lastwhile: Option<WhileContext>,
    ) {
        match self {
            Stmt::Break => {
                if let Some(context) = lastwhile {
                    let jump = data.dfg_mut().new_value().jump(context.while_end);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([jump]);
                    // needs a new basic block after this
                    *entry = data.dfg_mut().new_bb().basic_block(None);
                    let _ = data.layout_mut().bbs_mut().push_key_back(*entry);
                } else {
                    panic!("Break statement outside of loop");
                }
            }
            Stmt::Continue => {
                if let Some(context) = lastwhile {
                    let jump = data.dfg_mut().new_value().jump(context.while_cond);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([jump]);
                    // needs a new basic block after this
                    *entry = data.dfg_mut().new_bb().basic_block(None);
                    let _ = data.layout_mut().bbs_mut().push_key_back(*entry);
                } else {
                    panic!("Continue statement outside of loop");
                }
            }
            Stmt::While(exp, stmt) => {
                let mut while_cond = data.dfg_mut().new_bb().basic_block(None);
                let mut while_body = data.dfg_mut().new_bb().basic_block(None);
                let while_end = data.dfg_mut().new_bb().basic_block(None);
                let curwhile = WhileContext {
                    while_cond: while_cond.clone(),
                    while_end: while_end.clone(),
                };
                let curwhilebody = while_body.clone();
                let _ = data.layout_mut().bbs_mut().push_key_back(while_cond);
                let _ = data.layout_mut().bbs_mut().push_key_back(while_body);
                let _ = data.layout_mut().bbs_mut().push_key_back(while_end);
                let jumpcur = data.dfg_mut().new_value().jump(curwhile.while_cond);
                data.layout_mut()
                    .bb_mut(*entry)
                    .insts_mut()
                    .extend([jumpcur]);

                // while_cond
                let valexp = exp.gen_ir(data, &mut while_cond, var);
                let jumpexp =
                    data.dfg_mut()
                        .new_value()
                        .branch(valexp, curwhilebody, curwhile.while_end);
                data.layout_mut()
                    .bb_mut(while_cond)
                    .insts_mut()
                    .extend([jumpexp]);

                // while_body
                let mut myvar: HashMap<String, IdentValue> = var.clone();
                stmt.gen_ir(data, &mut while_body, &mut myvar, Some(curwhile.clone()));
                let jumpstmt = data.dfg_mut().new_value().jump(curwhile.while_cond);
                data.layout_mut()
                    .bb_mut(while_body)
                    .insts_mut()
                    .extend([jumpstmt]);

                // while_end
                *entry = while_end;
            }
            Stmt::Assign(id, exp) => {
                let val = id.gen_ir(data, entry, var, false);
                let valexp = exp.gen_ir(data, entry, var);
                let store = data.dfg_mut().new_value().store(valexp, val);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([store]);
                // let opt = var.get(id).cloned();
                // if let Some(IdentValue::Value(val)) = opt {
                // } else {
                //     panic!("trying to assign a function or a const var {}", id);
                // }
            }
            Stmt::Return(optexp) => {
                if let Some(exp) = (*optexp).as_ref() {
                    let val = exp.gen_ir(data, entry, var);
                    let ret = data.dfg_mut().new_value().ret(Some(val));
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([ret]);
                } else {
                    let ret = data.dfg_mut().new_value().ret(None);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([ret]);
                }
                // ret means a new basic block
                *entry = data.dfg_mut().new_bb().basic_block(None);
                let _ = data.layout_mut().bbs_mut().push_key_back(*entry);
            }
            Stmt::Do(optexp) => {
                if let Some(exp) = (*optexp).as_ref() {
                    let _ = exp.gen_ir(data, entry, var);
                } else {
                }
            }
            Stmt::Block(block) => {
                block.gen_ir(data, entry, var, lastwhile);
            }
            Stmt::If(exp, stmt) => {
                let val = exp.gen_ir(data, entry, var);
                let mut bb1 = data.dfg_mut().new_bb().basic_block(None);
                let bb3 = data.dfg_mut().new_bb().basic_block(None);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb1);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb3);
                let br1 = data.dfg_mut().new_value().branch(val, bb1, bb3);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([br1]);
                // if 里面
                let mut myvar: HashMap<String, IdentValue> = var.clone();
                stmt.gen_ir(data, &mut bb1, &mut myvar, lastwhile);
                let jto3 = data.dfg_mut().new_value().jump(bb3);
                data.layout_mut().bb_mut(bb1).insts_mut().extend([jto3]);
                // if 结束
                *entry = bb3;
            }
            Stmt::IfElse(exp, ifstmt, elsestmt) => {
                let val = exp.gen_ir(data, entry, var);
                let mut bb1 = data.dfg_mut().new_bb().basic_block(None);
                let mut bb2 = data.dfg_mut().new_bb().basic_block(None);
                let bb3 = data.dfg_mut().new_bb().basic_block(None);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb1);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb2);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb3);
                let br1 = data.dfg_mut().new_value().branch(val, bb1, bb2);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([br1]);
                // if, else 里面
                let mut myvar1: HashMap<String, IdentValue> = var.clone();
                let mut myvar2: HashMap<String, IdentValue> = var.clone();
                ifstmt.gen_ir(data, &mut bb1, &mut myvar1, lastwhile.clone());
                elsestmt.gen_ir(data, &mut bb2, &mut myvar2, lastwhile.clone());
                let jto3 = data.dfg_mut().new_value().jump(bb3);
                data.layout_mut().bb_mut(bb1).insts_mut().extend([jto3]);
                data.layout_mut().bb_mut(bb2).insts_mut().extend([jto3]);
                // if 结束
                *entry = bb3;
            }
        }
    }
}

impl Exp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        self.lorexp.gen_ir(data, entry, var)
    }
}

impl Lval {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
        needload: Bool,
    ) -> Value {
        match self {
            LVal::Ident(id) => {
                if let Some(val) = var.get(id).cloned() {
                    match val {
                        IdentValue::Func(_) => panic!("Function {} used as variable", id),
                        IdentValue::Value(val) => {
                            if needload {
                                let load = data.dfg_mut().new_value().load(val);
                                data.layout_mut().bb_mut(*entry).insts_mut().extend([load]);
                                return load;
                            } else {
                                return val;
                            }
                        }
                        IdentValue::ConstValue(val) => {
                            let nval = data.dfg_mut().new_value().integer(val);
                            return nval;
                        }
                    }
                } else {
                    panic!("Variable {} not found", id);
                }
            }
            LVal::Array(id, exps) => {
                if let Some(val) = var.get(id).cloned() {
                    match val {
                        IdentValue::Func(_) => panic!("Function {} used as array", id),
                        IdentValue::Value(val) => {
                            let mut ptrval = val;
                            for exp in exps.iter() {
                                let expval = exp.gen_ir(data, entry, var);
                                let newptr =
                                    data.dfg_mut().new_value().get_elem_ptr(ptrval, expval);
                                data.layout_mut()
                                    .bb_mut(*entry)
                                    .insts_mut()
                                    .extend([newptr]);
                                ptrval = newptr;
                            }
                            if needload {
                                let load = data.dfg_mut().new_value().load(ptrval);
                                data.layout_mut().bb_mut(*entry).insts_mut().extend([load]);
                                return load;
                            } else {
                                return val;
                            }
                        }
                        IdentValue::ConstValue(val) => {
                            panic!("Variable {} has a type that is not array", id);
                        }
                    }
                } else {
                    panic!("Variable {} not found", id);
                }
            }
        }
    }
}

impl PrimaryExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        match self {
            PrimaryExp::Exp(exp) => exp.gen_ir(data, entry, var),
            PrimaryExp::Number(num) => {
                let val = data.dfg_mut().new_value().integer(*num);
                val
            }
            PrimaryExp::LVal(lval) => {
                let val = lval.gen_ir(data, entry, var, true);
                val
            }
        }
    }
}

impl UnaryExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        match self {
            UnaryExp::PrimaryExp(primary_exp) => primary_exp.gen_ir(data, entry, var),
            UnaryExp::Pos(unary_exp) => unary_exp.gen_ir(data, entry, var),
            UnaryExp::Neg(unary_exp) => {
                let val = unary_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, val) {
                    return data.dfg_mut().new_value().integer(-rv);
                }
                let zero = data.dfg_mut().new_value().integer(0);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Sub, zero, val);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
            UnaryExp::Not(unary_exp) => {
                let val = unary_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, val) {
                    return data.dfg_mut().new_value().integer(!rv);
                }
                let zero = data.dfg_mut().new_value().integer(0);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Eq, zero, val);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
            UnaryExp::FuncCall(id, args) => {
                if let Some(undefval) = var.get(id).cloned() {
                    match undefval {
                        IdentValue::Func(func) => {
                            let mut argvec = Vec::new();
                            for arg in args.iter() {
                                let val = arg.gen_ir(data, entry, var);
                                argvec.push(val);
                            }
                            let call = data.dfg_mut().new_value().call(func, argvec);
                            data.layout_mut().bb_mut(*entry).insts_mut().extend([call]);
                            return call;
                        }
                        _ => panic!("Variable {} used as function", id),
                    }
                }
                panic!("function {} not found", id);
            }
        }
    }
}

impl MulExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        match self {
            MulExp::UnaryExp(unary_exp) => unary_exp.gen_ir(data, entry, var),
            MulExp::Mul(mul_exp, unary_exp) => {
                let left = mul_exp.gen_ir(data, entry, var);
                let right = unary_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer(rv * rv1);
                    }
                }
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Mul, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
            MulExp::Div(mul_exp, unary_exp) => {
                let left = mul_exp.gen_ir(data, entry, var);
                let right = unary_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer(rv / rv1);
                    }
                }
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Div, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
            MulExp::Mod(mul_exp, unary_exp) => {
                let left = mul_exp.gen_ir(data, entry, var);
                let right = unary_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer(rv % rv1);
                    }
                }
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Mod, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl AddExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        match self {
            AddExp::MulExp(mul_exp) => mul_exp.gen_ir(data, entry, var),
            AddExp::Add(add_exp, mul_exp) => {
                let left = add_exp.gen_ir(data, entry, var);
                let right = mul_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer(rv + rv1);
                    }
                }
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Add, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
            AddExp::Sub(add_exp, mul_exp) => {
                let left = add_exp.gen_ir(data, entry, var);
                let right = mul_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer(rv - rv1);
                    }
                }
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Sub, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl RelExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        match self {
            RelExp::AddExp(add_exp) => add_exp.gen_ir(data, entry, var),
            RelExp::Lt(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry, var);
                let right = add_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer((rv < rv1) as i32);
                    }
                }
                let res = data.dfg_mut().new_value().binary(BinaryOp::Lt, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
            RelExp::Le(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry, var);
                let right = add_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer((rv <= rv1) as i32);
                    }
                }
                let res = data.dfg_mut().new_value().binary(BinaryOp::Le, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
            RelExp::Gt(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry, var);
                let right = add_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer((rv > rv1) as i32);
                    }
                }
                let res = data.dfg_mut().new_value().binary(BinaryOp::Gt, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
            RelExp::Ge(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry, var);
                let right = add_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer((rv >= rv1) as i32);
                    }
                }
                let res = data.dfg_mut().new_value().binary(BinaryOp::Ge, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl EqExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        match self {
            EqExp::RelExp(rel_exp) => rel_exp.gen_ir(data, entry, var),
            EqExp::Eq(eq_exp, rel_exp) => {
                let left = eq_exp.gen_ir(data, entry, var);
                let right = rel_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer((rv == rv1) as i32);
                    }
                }
                let res = data.dfg_mut().new_value().binary(BinaryOp::Eq, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
            EqExp::Ne(eq_exp, rel_exp) => {
                let left = eq_exp.gen_ir(data, entry, var);
                let right = rel_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer((rv != rv1) as i32);
                    }
                }
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, left, right);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl LAndExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        match self {
            LAndExp::EqExp(eq_exp) => eq_exp.gen_ir(data, entry, var),
            LAndExp::And(l_and_exp, eq_exp) => {
                let zero = data.dfg_mut().new_value().integer(0);
                let left = l_and_exp.gen_ir(data, entry, var);
                if let Some(rv) = get_const_int(data, left) {
                    if rv == 0 {
                        return zero;
                    }
                    let right = eq_exp.gen_ir(data, entry, var);
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer((rv1 != 0) as i32);
                    }
                    let valright = data
                        .dfg_mut()
                        .new_value()
                        .binary(BinaryOp::NotEq, right, zero);
                    data.layout_mut()
                        .bb_mut(*entry)
                        .insts_mut()
                        .extend([valright]);
                    return valright;
                }
                let mut bb1 = data.dfg_mut().new_bb().basic_block(None);
                let bb3 = data.dfg_mut().new_bb().basic_block(None);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb1);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb3);
                let br1 = data.dfg_mut().new_value().branch(left, bb1, bb3);
                // 先定义一个结果变量，初始是 0
                let res = data.dfg_mut().new_value().alloc(Type::get_i32());
                let assign0 = data.dfg_mut().new_value().store(zero, res);
                data.layout_mut()
                    .bb_mut(*entry)
                    .insts_mut()
                    .extend([res, assign0, br1]);
                // bb1 就是计算 right
                let right = eq_exp.gen_ir(data, &mut bb1, var);
                let valright = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, right, zero);
                let assignright = data.dfg_mut().new_value().store(valright, res);
                let br2 = data.dfg_mut().new_value().jump(bb3);
                data.layout_mut()
                    .bb_mut(bb1)
                    .insts_mut()
                    .extend([valright, assignright, br2]);
                // bb3 返回 res
                *entry = bb3;
                let getres = data.dfg_mut().new_value().load(res);
                data.layout_mut().bb_mut(bb3).insts_mut().extend([getres]);
                return getres;
                //res 还必须 alloc
            }
        }
    }
}

impl LOrExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        match self {
            LOrExp::LAndExp(l_and_exp) => l_and_exp.gen_ir(data, entry, var),
            LOrExp::Or(l_or_exp, l_and_exp) => {
                let left = l_or_exp.gen_ir(data, entry, var);
                let one = data.dfg_mut().new_value().integer(1);
                let zero = data.dfg_mut().new_value().integer(0);
                if let Some(rv) = get_const_int(data, left) {
                    if rv == 1 {
                        return one;
                    }
                    let right = l_and_exp.gen_ir(data, entry, var);
                    if let Some(rv1) = get_const_int(data, right) {
                        return data.dfg_mut().new_value().integer((rv1 != 0) as i32);
                    }
                    let valright = data
                        .dfg_mut()
                        .new_value()
                        .binary(BinaryOp::NotEq, right, zero);
                    data.layout_mut()
                        .bb_mut(*entry)
                        .insts_mut()
                        .extend([valright]);
                    return valright;
                }
                let mut bb1 = data.dfg_mut().new_bb().basic_block(None);
                let bb3 = data.dfg_mut().new_bb().basic_block(None);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb1);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb3);
                let br1 = data.dfg_mut().new_value().branch(left, bb3, bb1);
                // 先定义一个结果变量，初始是 1
                let res = data.dfg_mut().new_value().alloc(Type::get_i32());
                let assign1 = data.dfg_mut().new_value().store(one, res);
                data.layout_mut()
                    .bb_mut(*entry)
                    .insts_mut()
                    .extend([res, assign1, br1]);
                // bb1 就是计算 right
                let right = l_and_exp.gen_ir(data, &mut bb1, var);
                let valright = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, right, zero);
                let assignright = data.dfg_mut().new_value().store(valright, res);
                let br2 = data.dfg_mut().new_value().jump(bb3);
                data.layout_mut()
                    .bb_mut(bb1)
                    .insts_mut()
                    .extend([valright, assignright, br2]);
                // bb3 返回 res
                *entry = bb3;
                let getres = data.dfg_mut().new_value().load(res);
                data.layout_mut().bb_mut(bb3).insts_mut().extend([getres]);
                return getres;
                //res 还必须 alloc
            }
        }
    }
}
