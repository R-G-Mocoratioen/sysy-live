use koopa::ir::dfg::*;
use koopa::ir::*;
use std::collections::HashMap;

pub enum Position {
    Stack(i32),
    RegX(i32),
}

pub struct BasicBlockVal {
    bb: BasicBlock,
    insts: Vec<Value>,
}

pub trait GenerateAsm {
    fn to_riscv(&mut self) -> String;
}

impl GenerateAsm for Program {
    fn to_riscv(&mut self) -> String {
        let mut ret = String::new();
        ret.push_str("  .data\n");
        ret.push_str("  .text\n");
        let mut funclist: Vec<Function> = Vec::new();
        for &func in self.func_layout() {
            funclist.push(func);
        }
        for func in funclist.iter() {
            ret.push_str(&self.func_mut(*func).to_riscv());
        }
        return ret;
    }
}

fn gen_lw_t_sp(ret: &mut String, id: i32, offset: i32) {
    if offset < 2048 {
        ret.push_str(&format!("  lw t{}, {}(sp)\n", id, offset));
    } else {
        ret.push_str(&format!("  li t{}, {}\n", id, offset));
        ret.push_str(&format!("  add t{}, sp, t{}\n", id, id));
        ret.push_str(&format!("  lw t{}, 0(t{})\n", id, id));
    }
}

fn maket(
    dfg: &DataFlowGraph,
    ret: &mut String,
    pos: &HashMap<Value, Position>,
    val: Value,
    id: i32,
) {
    let vkind = dfg.value(val).kind().clone();
    if let ValueKind::Integer(rval) = vkind {
        ret.push_str(&format!("  li t{}, {}\n", id, rval.value()));
        return;
    }
    let srcpos = pos.get(&val).unwrap();
    match srcpos {
        Position::Stack(offset) => {
            gen_lw_t_sp(ret, id.clone(), offset.clone());
        }
        Position::RegX(reg) => {
            ret.push_str(&format!("  mv t{}, x{}\n", id, reg));
        }
    }
    ret.push_str(&format!("  lw t{}, 0(t{})\n", id, id));
}

// might change the value of t1
fn store_t0_to_offset_using_t1(ret: &mut String, offset: i32) {
    if offset < 2048 {
        ret.push_str(&format!("  sw t0, {}(sp)\n", offset));
    } else {
        ret.push_str(&format!("  li t1, {}\n", offset));
        ret.push_str(&format!("  add t1, sp, t1\n"));
        ret.push_str(&format!("  sw t0, 0(t1)\n"));
    }
}

impl GenerateAsm for FunctionData {
    fn to_riscv(&mut self) -> String {
        let mut ret = String::new();
        ret.push_str("  .globl ");
        ret.push_str(&self.name()[1..]);
        ret.push_str("\n");
        ret.push_str(&self.name()[1..]);
        ret.push_str(":\n");
        let mut size_s: i32 = 0; // 局部变量
        let mut size_ra: i32 = 0; // 如果有 call，需要存储 ra 的值
        let mut size_a: i32 = 0;
        let mut bbids: Vec<BasicBlock> = Vec::new();
        let mut bbs: Vec<BasicBlockVal> = Vec::new();
        for (&bb, _) in self.layout_mut().bbs_mut().iter() {
            bbids.push(bb);
        }

        for &bb in bbids.iter() {
            let mut t = BasicBlockVal {
                bb: bb.clone(),
                insts: Vec::new(),
            };
            for (&inst, _) in self.layout_mut().bb_mut(bb).insts_mut().iter() {
                t.insts.push(inst.clone());
            }
            bbs.push(t);
        }

        for blockval in bbs.iter() {
            for inst in blockval.insts.iter() {
                let kind = self.dfg_mut().value(inst.clone()).kind().clone();
                if let ValueKind::Load(_) = kind {
                } else if let ValueKind::Store(_) = kind {
                } else if let ValueKind::Jump(_) = kind {
                } else if let ValueKind::Branch(_) = kind {
                } else if let ValueKind::Return(_) = kind {
                } else {
                    size_s += 4;
                }
                if let ValueKind::Alloc(_) = kind {
                    let ptrty = self.dfg_mut().value(inst.clone()).ty().clone();
                    match ptrty.kind() {
                        types::TypeKind::Pointer(t) => {
                            size_s += t.size() as i32;
                        }
                        _ => unreachable!(),
                    }
                }
                if let ValueKind::Call(call) = kind.clone() {
                    size_ra = 4;
                    let len = call.args().len() as i32;
                    if len > 8 {
                        size_a = std::cmp::max(4 * (len - 8), size_a);
                    }
                }
            }
        }
        // 1. 算出 sp
        let all_size = (size_s + size_ra + size_a + 15) / 16 * 16;
        if all_size != 0 {
            ret.push_str(&format!("addi sp, sp, -{}\n", all_size));
        }

        // 2. 把函数参数放到合适的位置
        let mut pos: HashMap<Value, Position> = HashMap::new();
        {
            let mut i = 0;
            for val0 in self.params() {
                let val = val0.clone();
                if i < 8 {
                    pos.insert(val, Position::RegX(10 + i));
                } else {
                    pos.insert(val, Position::Stack(all_size + 4 * (i - 8)));
                }
                i += 1;
            }
        }
        let mut bbid = 0;
        let mut bbids: HashMap<BasicBlock, i32> = HashMap::new();
        {
            for blockval in bbs.iter() {
                let bb = blockval.bb.clone();
                bbids.insert(bb, bbid);
                bbid += 1;
            }
        }

        // 3. 开始生成
        let mut curat = size_a;
        for blockval in bbs.iter() {
            // let bb = blockval.bb.clone();
            ret.push_str(&self.name()[1..]);
            ret.push_str(&format!("_{}:\n", bbids.get(&blockval.bb).unwrap()));
            for inst0 in blockval.insts.iter() {
                let inst = inst0.clone();
                let kind = self.dfg_mut().value(inst).kind().clone();
                match kind {
                    ValueKind::Alloc(_) => {
                        // sp + curat
                        let ptrty = self.dfg_mut().value(inst.clone()).ty().clone();
                        let sz;
                        match ptrty.kind() {
                            types::TypeKind::Pointer(t) => {
                                sz = t.size() as i32;
                            }
                            _ => unreachable!(),
                        }
                        curat += sz;
                        pos.insert(inst, Position::Stack(curat));
                        // 注意 pos 的含义是这个变量的值当前存在这里
                        if curat < 2048 {
                            ret.push_str(&format!("  addi t0, sp, {}\n", curat - sz));
                        } else {
                            ret.push_str(&format!("  li t0, {}\n", curat - sz));
                            ret.push_str(&format!("  add t0, sp, t0\n"));
                        }
                        store_t0_to_offset_using_t1(&mut ret, curat);
                        curat += 4;
                    }
                    ValueKind::Load(ld) => {
                        let src = ld.src();
                        maket(self.dfg(), &mut ret, &pos, src, 0);
                        store_t0_to_offset_using_t1(&mut ret, curat);
                        pos.insert(inst, Position::Stack(curat));
                        curat += 4;
                    }
                    ValueKind::Store(st) => {
                        let dst = st.dest();
                        let src = st.value();
                        maket(self.dfg(), &mut ret, &pos, src, 0);
                        maket(self.dfg(), &mut ret, &pos, dst, 1);
                        ret.push_str(&format!("  sw t0, 0(t1)\n"));
                    }
                    ValueKind::GetPtr(getptr) => {
                        let src = getptr.src();
                        let index = getptr.index();
                        maket(self.dfg(), &mut ret, &pos, src, 0);
                        maket(self.dfg(), &mut ret, &pos, index, 1);
                        let ptrty = self.dfg_mut().value(inst.clone()).ty().clone();
                        let sz;
                        match ptrty.kind() {
                            types::TypeKind::Pointer(t) => {
                                sz = t.size() as i32;
                            }
                            _ => unreachable!(),
                        }
                        ret.push_str(&format!("  li t2, {}\n", sz));
                        ret.push_str(&format!("  mul t1, t1, t2\n"));
                        ret.push_str(&format!("  add t0, t0, t1\n"));
                        store_t0_to_offset_using_t1(&mut ret, curat);
                        pos.insert(inst, Position::Stack(curat));
                        curat += 4;
                    }
                    ValueKind::GetElemPtr(gel) => {
                        let src = gel.src();
                        let index = gel.index();
                        maket(self.dfg(), &mut ret, &pos, src, 0);
                        maket(self.dfg(), &mut ret, &pos, index, 1);
                        let ptrty = self.dfg_mut().value(inst.clone()).ty().clone();
                        let sz;
                        match ptrty.kind() {
                            types::TypeKind::Pointer(t) => match t.kind() {
                                types::TypeKind::Array(t1, _) => {
                                    sz = t1.size() as i32;
                                }
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        }
                        ret.push_str(&format!("  li t2, {}\n", sz));
                        ret.push_str(&format!("  mul t1, t1, t2\n"));
                        ret.push_str(&format!("  add t0, t0, t1\n"));
                        store_t0_to_offset_using_t1(&mut ret, curat);
                        pos.insert(inst, Position::Stack(curat));
                        curat += 4;
                    }
                    ValueKind::Jump(jmp) => {
                        let bb = jmp.target();
                        let id = bbids.get(&bb).unwrap();
                        ret.push_str(&format!("  j "));
                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_{}\n", id));
                    }
                    ValueKind::Branch(br) => {
                        let cond = br.cond();
                        let bbtrue = br.true_bb();
                        let bbfalse = br.false_bb();
                        maket(self.dfg(), &mut ret, &pos, cond, 0);
                        ret.push_str(&format!("  beqz t0, "));
                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_{}\n", bbid));

                        ret.push_str(&format!("  j "));
                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_{}\n", bbids.get(&bbtrue).unwrap()));

                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_{}:\n", bbid));
                        bbid += 1;
                        ret.push_str(&format!("  j "));
                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_{}\n", bbids.get(&bbfalse).unwrap()));
                    }
                    ValueKind::Return(ret) => {
                        let val = ret.value();
                        match val {
                            Some(v) => {
                                maket(self.dfg(), &mut ret, &pos, v, 0);
                                ret.push_str(&format!("  mv a0, t0\n"));
                                ret.push_str(&format!("  ret\n"));
                            }
                            _ => {}
                        }
                    }
                    ValueKind::Call(call) => {}
                    ValueKind::Binary(bin) => {}
                    _ => {
                        panic!("bad value for instruction");
                    }
                }
            }
            bbid += 1;
        }
        if all_size != 0 {
            ret.push_str(&format!("addi sp, sp, {}\n", all_size));
        }
        return ret;
    }
}
