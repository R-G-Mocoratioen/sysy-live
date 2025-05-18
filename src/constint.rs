use koopa::ir::*;

pub fn get_const_int(data: &mut FunctionData, val: Value) -> Option<i32> {
    let vkind = data.dfg().value(val).kind().clone();
    if let ValueKind::Integer(rval) = vkind {
        return Some(rval.value());
    }
    None
}
