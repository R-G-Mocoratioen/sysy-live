use koopa::ir::*;

#[derive(Clone)]
pub enum IdentValue {
    Func(Function),
    Value(Value),
    FuncArgumentArray(Value, i32), // store the dimension to distinguish between value and pointer
    Array(Value, i32),
    ConstValue(i32),
}
