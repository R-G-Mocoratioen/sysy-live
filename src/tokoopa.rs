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
        let val = data.dfg_mut().new_value().integer(self.num);
        let ret = data.dfg_mut().new_value().ret(Some(val));
        data.layout_mut().bb_mut(entry).insts_mut().extend([ret]);
    }
}
