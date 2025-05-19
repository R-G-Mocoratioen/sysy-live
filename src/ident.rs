use koopa::ir::*;

#[derive(Clone)]
pub enum IdentValue {
    Func(Function),
    Value(Value),
    ConstValue(i32),
}
