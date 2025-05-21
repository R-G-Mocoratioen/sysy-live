use crate::ast::*;
use crate::constint::*;
use crate::ident::*;
use koopa::ir::builder::LocalInstBuilder;
use koopa::ir::builder::ValueBuilder;
use koopa::ir::*;
use std::collections::HashMap;

pub fn gen_arraytype(lens: Vec<i32>) -> Type {
    let mut curtype = Type::get_i32();
    for dim in lens.iter().rev() {
        curtype = Type::get_array(curtype, *dim as usize);
    }
    return curtype;
}

pub fn find_firstok(curpos: i32, lens: Vec<i32>) -> (i32, i32) {
    let mut i: i32 = lens.len() as i32 - 1;
    let mut firstok: i32 = i as i32;
    let mut cur = 1;
    let mut curcur = 1;
    while i >= 0 {
        cur = cur * lens[i as usize];
        if curpos % cur == 0 {
            firstok = i;
            curcur = cur;
        } else {
            break;
        }
        i -= 1;
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
        tmp /= i;
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
) -> Vec<i32> {
    let mut curpos = 0;
    let mut curvec: Vec<i32> = Vec::new();
    match initer {
        ArrayInit::Single(_) => panic!("using an exp to init a global array"),
        ArrayInit::Multiple(inits) => {
            for cur in inits.iter() {
                match cur.as_ref() {
                    ArrayInit::Single(exp) => {
                        let val = exp.gen_ir(data, entry, var);
                        if let Some(rv) = get_const_int(data, val) {
                            curvec.push(rv);
                        } else {
                            panic!("using a not const exp to init global array");
                        }
                        curpos += 1;
                    }
                    ArrayInit::Multiple(_) => {
                        if curpos % lens.last().unwrap() != 0 {
                            panic!("bad array initializer");
                        }
                        let respr = find_firstok(curpos, lens.clone());
                        let firstok: i32 = std::cmp::max(respr.0, 1);
                        // 用 inits 去匹配 firstok 之后的
                        let newvec = gen_globalarrayinit(
                            data,
                            entry,
                            var,
                            lens.clone()[firstok as usize..].to_vec(),
                            cur.as_ref().clone(),
                        );
                        curvec.extend(&newvec);
                        curpos += lens.clone()[firstok as usize..]
                            .to_vec()
                            .iter()
                            .fold(1, |acc, x| acc * x);
                    }
                }
            }
            let all = lens.iter().fold(1, |acc, x| acc * x);
            while curpos < all {
                curvec.push(0);
                curpos += 1;
            }
        }
    }
    return curvec;
}

pub fn gen_globalinitvalue(program: &mut Program, len: Vec<i32>, all: Vec<i32>) -> Value {
    if all.iter().all(|&x| x == 0) {
        return program.new_value().zero_init(gen_arraytype(len));
    }
    if len.len() == 1 {
        // for each x in len, map x into f(x)
        let mut elems: Vec<Value> = Vec::new();
        for v in all.iter() {
            let elem = program.new_value().integer(*v);
            elems.push(elem);
        }
        return program.new_value().aggregate(elems);
    }
    let l = len[0];
    let each = len.clone()[1..].to_vec().iter().fold(1, |acc, x| acc * x);
    let mut elems: Vec<Value> = Vec::new();
    for i in 0..l {
        let left: usize = i as usize * each as usize;
        let right: usize = (i as usize + 1) * each as usize;
        let res = gen_globalinitvalue(
            program,
            len.clone()[1..].to_vec(),
            all.clone()[left..right].to_vec(),
        );
        elems.push(res);
    }
    return program.new_value().aggregate(elems);
}
