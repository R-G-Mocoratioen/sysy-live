use koopa::ir::*;

#[derive(Clone)]
pub struct WhileContext {
    pub while_cond: BasicBlock,
    pub while_end: BasicBlock,
}
