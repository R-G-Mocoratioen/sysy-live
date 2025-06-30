use crate::arrayinit::*;
use crate::ast::*;
use crate::constint::*;
use crate::gen_music::*;
use crate::ident::*;
use crate::whilecontext::*;
use koopa::ir::builder::LocalInstBuilder;
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

        // now comes the music decls
        self.adddecl(
            program,
            var,
            "score_sing".into(),
            Type::get_unit(),
            vec![
                Type::get_i32(),
                Type::get_pointer(Type::get_i32()),
                Type::get_pointer(Type::get_i32()),
                Type::get_i32(),
                Type::get_i32(),
                Type::get_i32(),
            ],
        );
        self.adddecl(
            program,
            var,
            "score_push".into(),
            Type::get_unit(),
            vec![Type::get_i32(), Type::get_i32()],
        );
        self.adddecl(
            program,
            var,
            "bar_push".into(),
            Type::get_unit(),
            vec![Type::get_i32(), Type::get_i32()],
        );
        self.adddecl(
            program,
            var,
            "newnote".into(),
            Type::get_i32(),
            vec![Type::get_i32(), Type::get_i32(), Type::get_i32()],
        );
        self.adddecl(
            program,
            var,
            "newnote_rest".into(),
            Type::get_i32(),
            vec![Type::get_i32(), Type::get_i32(), Type::get_i32()],
        );
        self.adddecl(program, var, "newbar".into(), Type::get_i32(), vec![]);
        self.adddecl(program, var, "newscore".into(), Type::get_i32(), vec![]);
        self.adddecl(
            program,
            var,
            "bar_setbpm".into(),
            Type::get_unit(),
            vec![Type::get_i32(), Type::get_i32()],
        );
        self.adddecl(
            program,
            var,
            "score_setbpm".into(),
            Type::get_unit(),
            vec![Type::get_i32(), Type::get_i32()],
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
                                arrayinit.as_ref().clone(),
                            );
                            initmap.insert(id.clone(), res);
                        }
                        _ => panic!("no global music declarations is allowed"),
                    }
                }
            }
            // 还要处理函数里的数组维度问题！
            for func in self.func_defs.iter() {
                for param in func.params.iter() {
                    match param {
                        FuncParam::Var(_) => {}
                        FuncParam::Array(paramid, exps) => {
                            let mut lens: Vec<i32> = Vec::new();
                            for exp in exps.iter() {
                                let val = exp.gen_ir(pseudo_data, &mut entry, &mut var);
                                if let Some(res) = get_const_int(pseudo_data, val) {
                                    lens.push(res);
                                } else {
                                    panic!("global array size not a constant");
                                }
                            }
                            let newparamid = func.id.clone() + "!" + paramid;
                            sizemap.insert(newparamid.clone(), lens.clone());
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
                            let typ = gen_arraytype(len.clone());
                            let zero = program.new_value().zero_init(typ);
                            let alloc = program.new_value().global_alloc(zero);
                            var.insert(id.clone(), IdentValue::Array(alloc, len.len() as i32));
                        }
                        VarDef::ArrayInit(id, _, _) => {
                            let len = sizemap.get(id).unwrap().clone();
                            let initv = gen_globalinitvalue(
                                &mut program,
                                len.clone(),
                                initmap.get(id).unwrap().clone(),
                            );
                            let alloc = program.new_value().global_alloc(initv);
                            var.insert(id.clone(), IdentValue::Array(alloc, len.len() as i32));
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        println!("successfully built global vars");

        self.gen_libfuncs(&mut program, &mut var);

        for func in self.func_defs.iter() {
            let mut typevec = Vec::new();

            for param in func.params.iter() {
                // typevec.push(Type::get_i32()); // 目前只有 int 一个类型
                match param {
                    FuncParam::Var(_) => {
                        typevec.push(Type::get_i32());
                    }
                    FuncParam::Array(paramid, _) => {
                        let newparamid = func.id.clone() + "!" + paramid;
                        if let Some(lens) = sizemap.get(&newparamid) {
                            let curtype = gen_arraytype(lens.clone());
                            typevec.push(Type::get_pointer(curtype));
                        } else {
                            panic!("function {} param {} not found", func.id, paramid);
                        }
                    }
                }
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
            let funcparamval = main_data.params()[id];
            let paramtyp = main_data.dfg().value(funcparamval).ty().clone();
            let alloc = main_data.dfg_mut().new_value().alloc(paramtyp);
            let store = main_data.dfg_mut().new_value().store(funcparamval, alloc);
            main_data
                .layout_mut()
                .bb_mut(entry)
                .insts_mut()
                .extend([alloc, store]);
            match param {
                FuncParam::Var(paramid) => {
                    myvar.insert(paramid.clone(), IdentValue::Value(alloc));
                }
                FuncParam::Array(paramid, exps) => {
                    myvar.insert(
                        paramid.clone(),
                        IdentValue::FuncArgumentArray(alloc, exps.len() as i32 + 1),
                    );
                }
            }
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
            ArrayInit::Single(_) => panic!("using an exp to init an array"),
            ArrayInit::Multiple(inits) => {
                for cur in inits.iter() {
                    match cur.as_ref() {
                        ArrayInit::Single(exp) => {
                            let val = exp.gen_ir(data, entry, var);
                            let at = gen_arrayelem_ptr(data, entry, alloc, curpos, lens.clone());
                            let store = data.dfg_mut().new_value().store(val, at);
                            data.layout_mut().bb_mut(*entry).insts_mut().extend([store]);
                            curpos += 1;
                        }
                        ArrayInit::Multiple(_) => {
                            if curpos % lens.last().unwrap() != 0 {
                                panic!("bad array initializer");
                            }
                            let respr = find_firstok(curpos, lens.clone());
                            let firstok: i32 = std::cmp::max(respr.0, 1);
                            let curcur = respr.1;
                            // 用 inits 去匹配 firstok 之后的
                            let at = gen_arrayelem_ptr(
                                data,
                                entry,
                                alloc,
                                curpos / curcur,
                                lens.clone()[0..firstok as usize].to_vec(),
                            );
                            cur.gen_ir(
                                data,
                                entry,
                                var,
                                at,
                                lens.clone()[firstok as usize..].to_vec(),
                            );
                            curpos += lens.clone()[firstok as usize..]
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
                let curtype = gen_arraytype(lens);
                let alloc = data.dfg_mut().new_value().alloc(curtype);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([alloc]);
                var.insert(id.clone(), IdentValue::Array(alloc, exps.len() as i32));
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
                let curtype = gen_arraytype(lens.clone());
                let alloc = data.dfg_mut().new_value().alloc(curtype);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([alloc]);
                var.insert(id.clone(), IdentValue::Array(alloc, exps.len() as i32));
                arrayinit.gen_ir(data, entry, var, alloc, lens.clone());
            }
            VarDef::ScoreInit(id, score) => {
                let it = gen_score(data, entry, var, score);
                let alloc = data.dfg_mut().new_value().alloc(Type::get_i32());
                let store = data.dfg_mut().new_value().store(it, alloc);
                data.layout_mut()
                    .bb_mut(*entry)
                    .insts_mut()
                    .extend([alloc, store]);
                var.insert(id.clone(), IdentValue::Value(alloc));
            }
            VarDef::BarInit(id, bar) => {
                let it = gen_bar(data, entry, var, bar);
                let alloc = data.dfg_mut().new_value().alloc(Type::get_i32());
                let store = data.dfg_mut().new_value().store(it, alloc);
                data.layout_mut()
                    .bb_mut(*entry)
                    .insts_mut()
                    .extend([alloc, store]);
                var.insert(id.clone(), IdentValue::Value(alloc));
            }
            VarDef::NoteInit(id, note, fz, fm) => {
                let it = gen_note(data, entry, var, (**note).clone(), fz.clone(), fm.clone());
                let alloc = data.dfg_mut().new_value().alloc(Type::get_i32());
                let store = data.dfg_mut().new_value().store(it, alloc);
                data.layout_mut()
                    .bb_mut(*entry)
                    .insts_mut()
                    .extend([alloc, store]);
                var.insert(id.clone(), IdentValue::Value(alloc));
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
            Stmt::Sing(_id, _name1, _name2, _name3, _name4, _name5) => {
                let id = _id.gen_ir(data, entry, var);
                let name1 = _name1.gen_ir(data, entry, var);
                let name2 = _name2.gen_ir(data, entry, var);
                let name3 = _name3.gen_ir(data, entry, var);
                let name4 = _name4.gen_ir(data, entry, var);
                let name5 = _name5.gen_ir(data, entry, var);
                let func = var.get("score_sing".into()).unwrap().clone();
                if let IdentValue::Func(func) = func {
                    let call = data
                        .dfg_mut()
                        .new_value()
                        .call(func, vec![id, name1, name2, name3, name4, name5]);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([call]);
                } else {
                    panic!("score_sing is not a function");
                }
            }
            Stmt::PushBar(_id, _num) => {
                let id = _id.gen_ir(data, entry, var);
                let num = _num.gen_ir(data, entry, var);
                let func = var.get("score_push".into()).unwrap().clone();
                if let IdentValue::Func(func) = func {
                    let call = data.dfg_mut().new_value().call(func, vec![id, num]);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([call]);
                } else {
                    panic!("bar_push is not a function");
                }
            }
            Stmt::PushNote(_id, _num) => {
                let id = _id.gen_ir(data, entry, var);
                let num = _num.gen_ir(data, entry, var);
                let func = var.get("bar_push".into()).unwrap().clone();
                if let IdentValue::Func(func) = func {
                    let call = data.dfg_mut().new_value().call(func, vec![id, num]);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([call]);
                } else {
                    panic!("bar_push is not a function");
                }
            }
            Stmt::SetBarBpm(_id, _num) => {
                let id = _id.gen_ir(data, entry, var);
                let num = _num.gen_ir(data, entry, var);
                let func = var.get("bar_setbpm".into()).unwrap().clone();
                if let IdentValue::Func(func) = func {
                    let call = data.dfg_mut().new_value().call(func, vec![id, num]);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([call]);
                } else {
                    panic!("bar_setbpm is not a function");
                }
            }
            Stmt::SetScoreBpm(_id, _num) => {
                let id = _id.gen_ir(data, entry, var);
                let num = _num.gen_ir(data, entry, var);
                let func = var.get("score_setbpm".into()).unwrap().clone();
                if let IdentValue::Func(func) = func {
                    let call = data.dfg_mut().new_value().call(func, vec![id, num]);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([call]);
                } else {
                    panic!("score_setbpm is not a function");
                }
            }
        }
    }
}

impl Exp {
    pub fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
    ) -> Value {
        self.lorexp.gen_ir(data, entry, var)
    }
}

impl LVal {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, IdentValue>,
        needload: bool,
    ) -> Value {
        // 左值不需要 needload，右值就需要 load
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
                        IdentValue::FuncArgumentArray(val, _) => {
                            if !needload {
                                panic!("Array {} used as left value", id);
                            }
                            // 这里的 val 就是和 func 的 val 一样的传法
                            let load = data.dfg_mut().new_value().load(val);
                            data.layout_mut().bb_mut(*entry).insts_mut().extend([load]);
                            return load;
                        }
                        IdentValue::Array(val, _) => {
                            if !needload {
                                panic!("Array {} used as left value", id);
                            }
                            let zero = data.dfg_mut().new_value().integer(0);
                            let load = data.dfg_mut().new_value().get_elem_ptr(val, zero);
                            data.layout_mut().bb_mut(*entry).insts_mut().extend([load]);
                            return load;
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
                        IdentValue::Value(_) => panic!("Variable {} used as array", id),
                        IdentValue::FuncArgumentArray(val0, dim) => {
                            let load0 = data.dfg_mut().new_value().load(val0);
                            data.layout_mut().bb_mut(*entry).insts_mut().extend([load0]);
                            let mut ptrval = load0;
                            let mut fir = true;
                            for exp in exps.iter() {
                                let expval = exp.gen_ir(data, entry, var);
                                let newptr: Value;
                                if fir {
                                    newptr = data.dfg_mut().new_value().get_ptr(ptrval, expval);
                                } else {
                                    newptr =
                                        data.dfg_mut().new_value().get_elem_ptr(ptrval, expval);
                                }
                                data.layout_mut()
                                    .bb_mut(*entry)
                                    .insts_mut()
                                    .extend([newptr]);
                                ptrval = newptr;
                                fir = false;
                            }
                            if needload {
                                let load: Value;
                                let zero = data.dfg_mut().new_value().integer(0);
                                if exps.len() as i32 == dim {
                                    load = data.dfg_mut().new_value().load(ptrval);
                                } else {
                                    load = data.dfg_mut().new_value().get_elem_ptr(ptrval, zero);
                                }
                                data.layout_mut().bb_mut(*entry).insts_mut().extend([load]);
                                return load;
                            } else {
                                if exps.len() as i32 != dim {
                                    panic!("Array pointer {} used as left value", id);
                                }
                                return ptrval;
                            }
                        }
                        IdentValue::Array(val, dim) => {
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
                                let load: Value;
                                let zero = data.dfg_mut().new_value().integer(0);
                                if exps.len() as i32 == dim {
                                    load = data.dfg_mut().new_value().load(ptrval);
                                } else {
                                    load = data.dfg_mut().new_value().get_elem_ptr(ptrval, zero);
                                }
                                data.layout_mut().bb_mut(*entry).insts_mut().extend([load]);
                                return load;
                            } else {
                                if exps.len() as i32 != dim {
                                    panic!("Array pointer {} used as left value", id);
                                }
                                return ptrval;
                            }
                        }
                        IdentValue::ConstValue(_) => {
                            panic!("Const Variable {} is not array", id);
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
                    return data.dfg_mut().new_value().integer((rv == 0) as i32);
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
                    if rv != 0 {
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
