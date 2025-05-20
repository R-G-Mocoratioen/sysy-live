use koopa::ir::*;

#[derive(Clone)]
pub enum IdentValue {
    Func(Function),
    Value(Value),
    Array(Value),
    ConstValue(i32),
}
