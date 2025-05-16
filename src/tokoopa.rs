use crate::ast::*;
use koopa::ir::builder_traits::*;
use koopa::ir::*;
use std::collections::HashMap;

impl CompUnit {
    pub fn gen_ir(&self) -> Program {
        let mut program = Program::new();

        self.func_def.gen_ir(&mut program);

        program
    }
}

impl FuncDef {
    fn gen_ir(&self, program: &mut Program) {
        let main = program.new_func(FunctionData::new(
            ("@".to_owned() + &self.id).into(),
            Vec::new(),
            Type::get_i32(),
        ));

        let main_data = program.func_mut(main);
        let mut var: HashMap<String, Value> = HashMap::new();

        let mut entry = main_data.dfg_mut().new_bb().basic_block(None);
        let _ = main_data.layout_mut().bbs_mut().push_key_back(entry);

        self.block.gen_ir(main_data, &mut entry, &mut var);

        // add an unreachable "return 0" at the end

        let zero = main_data.dfg_mut().new_value().integer(0);
        let ret = main_data.dfg_mut().new_value().ret(Some(zero));
        main_data
            .layout_mut()
            .bb_mut(entry)
            .insts_mut()
            .extend([ret]);
    }
}

impl VarDef {
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock, var: &mut HashMap<String, Value>) {
        let alloc = data.dfg_mut().new_value().alloc(Type::get_i32());
        data.layout_mut().bb_mut(entry).insts_mut().extend([alloc]);
        match self {
            VarDef::Ident(id) => {
                var.insert(id.clone(), alloc);
            }
            VarDef::IdentInit(id, exp) => {
                var.insert(id.clone(), alloc);
                let val = exp.gen_ir(data, entry, var);
                let store = data.dfg_mut().new_value().store(val, alloc);
                data.layout_mut().bb_mut(entry).insts_mut().extend([store]);
            }
        }
    }
}

impl Decl {
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock, var: &mut HashMap<String, Value>) {
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
        var: &mut HashMap<String, Value>,
    ) {
        let mut myvar: HashMap<String, Value> = var.clone();
        for item in &self.vecitem {
            item.gen_ir(data, entry, &mut myvar);
        }
    }
}

impl BlockItem {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, Value>,
    ) {
        match self {
            BlockItem::Stmt(stmt) => stmt.gen_ir(data, entry, var),
            BlockItem::Decl(decl) => decl.gen_ir(data, *entry, var),
        }
    }
}

impl Stmt {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: &mut BasicBlock,
        var: &mut HashMap<String, Value>,
    ) {
        match self {
            Stmt::Assign(id, exp) => {
                let opt = var.get(id).cloned();
                if let Some(val) = opt {
                    let valexp = exp.gen_ir(data, *entry, var);
                    let store = data.dfg_mut().new_value().store(valexp, val);
                    data.layout_mut().bb_mut(*entry).insts_mut().extend([store]);
                }
            }
            Stmt::Return(optexp) => {
                if let Some(exp) = (*optexp).as_ref() {
                    let val = exp.gen_ir(data, *entry, var);
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
                    let _ = exp.gen_ir(data, *entry, var);
                } else {
                }
            }
            Stmt::Block(block) => {
                block.gen_ir(data, entry, var);
            }
            Stmt::If(exp, stmt) => {
                let val = exp.gen_ir(data, *entry, var);
                let mut bb1 = data.dfg_mut().new_bb().basic_block(None);
                let bb3 = data.dfg_mut().new_bb().basic_block(None);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb1);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb3);
                let br1 = data.dfg_mut().new_value().branch(val, bb1, bb3);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([br1]);
                // if 里面
                let mut myvar: HashMap<String, Value> = var.clone();
                stmt.gen_ir(data, &mut bb1, &mut myvar);
                let jto3 = data.dfg_mut().new_value().jump(bb3);
                data.layout_mut().bb_mut(bb1).insts_mut().extend([jto3]);
                // if 结束
                *entry = bb3;
            }
            Stmt::IfElse(exp, ifstmt, elsestmt) => {
                let val = exp.gen_ir(data, *entry, var);
                let mut bb1 = data.dfg_mut().new_bb().basic_block(None);
                let mut bb2 = data.dfg_mut().new_bb().basic_block(None);
                let bb3 = data.dfg_mut().new_bb().basic_block(None);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb1);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb2);
                let _ = data.layout_mut().bbs_mut().push_key_back(bb3);
                let br1 = data.dfg_mut().new_value().branch(val, bb1, bb2);
                data.layout_mut().bb_mut(*entry).insts_mut().extend([br1]);
                // if, else 里面
                let mut myvar1: HashMap<String, Value> = var.clone();
                let mut myvar2: HashMap<String, Value> = var.clone();
                ifstmt.gen_ir(data, &mut bb1, &mut myvar1);
                elsestmt.gen_ir(data, &mut bb2, &mut myvar2);
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
        entry: BasicBlock,
        var: &mut HashMap<String, Value>,
    ) -> Value {
        self.lorexp.gen_ir(data, entry, var)
    }
}

impl PrimaryExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: BasicBlock,
        var: &mut HashMap<String, Value>,
    ) -> Value {
        match self {
            PrimaryExp::Exp(exp) => exp.gen_ir(data, entry, var),
            PrimaryExp::Number(num) => {
                let val = data.dfg_mut().new_value().integer(*num);
                val
            }
            PrimaryExp::LVal(id) => {
                if let Some(val) = var.get(id) {
                    let load = data.dfg_mut().new_value().load(*val);
                    data.layout_mut().bb_mut(entry).insts_mut().extend([load]);
                    load
                } else {
                    panic!("Variable {} not found", id);
                }
            }
        }
    }
}

impl UnaryExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: BasicBlock,
        var: &mut HashMap<String, Value>,
    ) -> Value {
        match self {
            UnaryExp::PrimaryExp(primary_exp) => primary_exp.gen_ir(data, entry, var),
            UnaryExp::Pos(unary_exp) => unary_exp.gen_ir(data, entry, var),
            UnaryExp::Neg(unary_exp) => {
                let zero = data.dfg_mut().new_value().integer(0);
                let val = unary_exp.gen_ir(data, entry, var);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Sub, zero, val);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            UnaryExp::Not(unary_exp) => {
                let zero = data.dfg_mut().new_value().integer(0);
                let val = unary_exp.gen_ir(data, entry, var);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Eq, zero, val);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl MulExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: BasicBlock,
        var: &mut HashMap<String, Value>,
    ) -> Value {
        match self {
            MulExp::UnaryExp(unary_exp) => unary_exp.gen_ir(data, entry, var),
            MulExp::Mul(mul_exp, unary_exp) => {
                let left = mul_exp.gen_ir(data, entry, var);
                let right = unary_exp.gen_ir(data, entry, var);
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Mul, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            MulExp::Div(mul_exp, unary_exp) => {
                let left = mul_exp.gen_ir(data, entry, var);
                let right = unary_exp.gen_ir(data, entry, var);
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Div, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            MulExp::Mod(mul_exp, unary_exp) => {
                let left = mul_exp.gen_ir(data, entry, var);
                let right = unary_exp.gen_ir(data, entry, var);
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Mod, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl AddExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: BasicBlock,
        var: &mut HashMap<String, Value>,
    ) -> Value {
        match self {
            AddExp::MulExp(mul_exp) => mul_exp.gen_ir(data, entry, var),
            AddExp::Add(add_exp, mul_exp) => {
                let left = add_exp.gen_ir(data, entry, var);
                let right = mul_exp.gen_ir(data, entry, var);
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Add, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            AddExp::Sub(add_exp, mul_exp) => {
                let left = add_exp.gen_ir(data, entry, var);
                let right = mul_exp.gen_ir(data, entry, var);
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Sub, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl RelExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: BasicBlock,
        var: &mut HashMap<String, Value>,
    ) -> Value {
        match self {
            RelExp::AddExp(add_exp) => add_exp.gen_ir(data, entry, var),
            RelExp::Lt(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry, var);
                let right = add_exp.gen_ir(data, entry, var);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Lt, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            RelExp::Le(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry, var);
                let right = add_exp.gen_ir(data, entry, var);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Le, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            RelExp::Gt(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry, var);
                let right = add_exp.gen_ir(data, entry, var);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Gt, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            RelExp::Ge(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry, var);
                let right = add_exp.gen_ir(data, entry, var);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Ge, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl EqExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: BasicBlock,
        var: &mut HashMap<String, Value>,
    ) -> Value {
        match self {
            EqExp::RelExp(rel_exp) => rel_exp.gen_ir(data, entry, var),
            EqExp::Eq(eq_exp, rel_exp) => {
                let left = eq_exp.gen_ir(data, entry, var);
                let right = rel_exp.gen_ir(data, entry, var);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Eq, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            EqExp::Ne(eq_exp, rel_exp) => {
                let left = eq_exp.gen_ir(data, entry, var);
                let right = rel_exp.gen_ir(data, entry, var);
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl LAndExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: BasicBlock,
        var: &mut HashMap<String, Value>,
    ) -> Value {
        match self {
            LAndExp::EqExp(eq_exp) => eq_exp.gen_ir(data, entry, var),
            LAndExp::And(l_and_exp, eq_exp) => {
                let left = l_and_exp.gen_ir(data, entry, var);
                let right = eq_exp.gen_ir(data, entry, var);
                let zero = data.dfg_mut().new_value().integer(0);
                let t1 = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, left, zero);
                let t2 = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, right, zero);
                let t3 = data.dfg_mut().new_value().binary(BinaryOp::And, t1, t2);
                data.layout_mut()
                    .bb_mut(entry)
                    .insts_mut()
                    .extend([t1, t2, t3]);
                t3
            }
        }
    }
}

impl LOrExp {
    fn gen_ir(
        &self,
        data: &mut FunctionData,
        entry: BasicBlock,
        var: &mut HashMap<String, Value>,
    ) -> Value {
        match self {
            LOrExp::LAndExp(l_and_exp) => l_and_exp.gen_ir(data, entry, var),
            LOrExp::Or(l_or_exp, l_and_exp) => {
                let left = l_or_exp.gen_ir(data, entry, var);
                let right = l_and_exp.gen_ir(data, entry, var);
                let zero = data.dfg_mut().new_value().integer(0);
                let t1 = data.dfg_mut().new_value().binary(BinaryOp::Or, left, right);
                let t2 = data.dfg_mut().new_value().binary(BinaryOp::NotEq, zero, t1);
                data.layout_mut().bb_mut(entry).insts_mut().extend([t1, t2]);
                t2
            }
        }
    }
}
