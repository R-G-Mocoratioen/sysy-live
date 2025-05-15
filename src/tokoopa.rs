use crate::ast::*;
use koopa::ir::builder_traits::*;
use koopa::ir::*;

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

        self.block.gen_ir(main_data);
    }
}

impl Block {
    fn gen_ir(&self, data: &mut FunctionData) {
        let entry = data.dfg_mut().new_bb().basic_block(Some("%entry".into()));
        let _ = data.layout_mut().bbs_mut().push_key_back(entry);

        self.stmt.gen_ir(data, entry);
    }
}

impl Stmt {
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) {
        // let val = data.dfg_mut().new_value().integer(self.num);
        let val = self.exp.gen_ir(data, entry);
        let ret = data.dfg_mut().new_value().ret(Some(val));
        data.layout_mut().bb_mut(entry).insts_mut().extend([ret]);
    }
}

impl Exp {
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) -> Value {
        self.lorexp.gen_ir(data, entry)
    }
}

impl PrimaryExp {
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) -> Value {
        match self {
            PrimaryExp::Exp(exp) => exp.gen_ir(data, entry),
            PrimaryExp::Number(num) => {
                let val = data.dfg_mut().new_value().integer(*num);
                val
            }
        }
    }
}

impl UnaryExp {
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) -> Value {
        match self {
            UnaryExp::PrimaryExp(primary_exp) => primary_exp.gen_ir(data, entry),
            UnaryExp::Pos(unary_exp) => unary_exp.gen_ir(data, entry),
            UnaryExp::Neg(unary_exp) => {
                let zero = data.dfg_mut().new_value().integer(0);
                let val = unary_exp.gen_ir(data, entry);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Sub, zero, val);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            UnaryExp::Not(unary_exp) => {
                let zero = data.dfg_mut().new_value().integer(0);
                let val = unary_exp.gen_ir(data, entry);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Eq, zero, val);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl MulExp {
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) -> Value {
        match self {
            MulExp::UnaryExp(unary_exp) => unary_exp.gen_ir(data, entry),
            MulExp::Mul(mul_exp, unary_exp) => {
                let left = mul_exp.gen_ir(data, entry);
                let right = unary_exp.gen_ir(data, entry);
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Mul, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            MulExp::Div(mul_exp, unary_exp) => {
                let left = mul_exp.gen_ir(data, entry);
                let right = unary_exp.gen_ir(data, entry);
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Div, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            MulExp::Mod(mul_exp, unary_exp) => {
                let left = mul_exp.gen_ir(data, entry);
                let right = unary_exp.gen_ir(data, entry);
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
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) -> Value {
        match self {
            AddExp::MulExp(mul_exp) => mul_exp.gen_ir(data, entry),
            AddExp::Add(add_exp, mul_exp) => {
                let left = add_exp.gen_ir(data, entry);
                let right = mul_exp.gen_ir(data, entry);
                let res = data
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Add, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            AddExp::Sub(add_exp, mul_exp) => {
                let left = add_exp.gen_ir(data, entry);
                let right = mul_exp.gen_ir(data, entry);
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
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) -> Value {
        match self {
            RelExp::AddExp(add_exp) => add_exp.gen_ir(data, entry),
            RelExp::Lt(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry);
                let right = add_exp.gen_ir(data, entry);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Lt, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            RelExp::Le(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry);
                let right = add_exp.gen_ir(data, entry);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Le, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            RelExp::Gt(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry);
                let right = add_exp.gen_ir(data, entry);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Gt, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            RelExp::Ge(rel_exp, add_exp) => {
                let left = rel_exp.gen_ir(data, entry);
                let right = add_exp.gen_ir(data, entry);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Ge, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
        }
    }
}

impl EqExp {
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) -> Value {
        match self {
            EqExp::RelExp(rel_exp) => rel_exp.gen_ir(data, entry),
            EqExp::Eq(eq_exp, rel_exp) => {
                let left = eq_exp.gen_ir(data, entry);
                let right = rel_exp.gen_ir(data, entry);
                let res = data.dfg_mut().new_value().binary(BinaryOp::Eq, left, right);
                data.layout_mut().bb_mut(entry).insts_mut().extend([res]);
                res
            }
            EqExp::Ne(eq_exp, rel_exp) => {
                let left = eq_exp.gen_ir(data, entry);
                let right = rel_exp.gen_ir(data, entry);
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
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) -> Value {
        match self {
            LAndExp::EqExp(eq_exp) => eq_exp.gen_ir(data, entry),
            LAndExp::And(l_and_exp, eq_exp) => {
                let left = l_and_exp.gen_ir(data, entry);
                let right = eq_exp.gen_ir(data, entry);
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
    fn gen_ir(&self, data: &mut FunctionData, entry: BasicBlock) -> Value {
        match self {
            LOrExp::LAndExp(l_and_exp) => l_and_exp.gen_ir(data, entry),
            LOrExp::Or(l_or_exp, l_and_exp) => {
                let left = l_or_exp.gen_ir(data, entry);
                let right = l_and_exp.gen_ir(data, entry);
                let zero = data.dfg_mut().new_value().integer(0);
                let t1 = data.dfg_mut().new_value().binary(BinaryOp::Or, left, right);
                let t2 = data.dfg_mut().new_value().binary(BinaryOp::NotEq, zero, t1);
                data.layout_mut().bb_mut(entry).insts_mut().extend([t1, t2]);
                t2
            }
        }
    }
}
