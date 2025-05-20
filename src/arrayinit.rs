use crate::ast::*;
use crate::tokoopa::*;
use koopa::ir::*;

pub enum GlobalArrayInit {
    Single(i32),
    ZeroInit,
    Multiple(Vec<Box<GlobalArrayInit>>),
}

pub fn find_firstok(curpos: i32, lens: Vec<i32>) -> (i32, i32) {
    let mut i: i32 = lens.len() - 1;
    let mut firstok: i32 = i;
    let mut cur = 1;
    let mut curcur = 1;
    while i >= 0 {
        cur = cur * lens[i];
        if curpos % cur == 0 {
            firstok = i;
            curcur = cur;
        } else {
            break;
        }
    }
    return (firstok, curcur);
}

// gen_arrayelem_ptr(data, entry, alloc, *curpos, lens.clone());
pub fn gen_arrayelem_ptr(
    data: &mut FunctionData,
    entry: &mut BasicBlock,
    alloc: Value,
    curpos: i32,
    lens: Vec<i32>,
) -> Value {
    let mut tmp = curpos;
    let mut ats: Vec<i32> = Vec::new();
    for i in lens.iter().rev() {
        ats.push(tmp % i.clone());
    }
    let mut ptrval = alloc;
    for i in ats.iter().rev() {
        let indexval = data.dfg_mut().new_value().integer(i.clone());
        let newptr = data.dfg_mut().new_value().get_elem_ptr(ptrval, indexval);
        data.layout_mut()
            .bb_mut(*entry)
            .insts_mut()
            .extend([newptr]);
        ptrval = newptr;
    }
    return ptrval;
}

pub fn gen_globalarrayinit(
    data: &mut FunctionData,
    entry: &mut BasicBlock,
    var: &mut HashMap<String, IdentValue>,
    lens: Vec<i32>,
    initer: ArrayInit,
) -> GlobalArrayInit {
    let mut curpos = 0;
    match initer {
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
                        let mut tmp: i32 = 0;
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
