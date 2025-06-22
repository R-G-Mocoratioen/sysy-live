use koopa::ir::dfg::*;
use koopa::ir::*;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Position {
    Stack(i32),
    RegX(i32),
    Global(String), // name and type
}

pub struct BasicBlockVal {
    pub bb: BasicBlock,
    pub insts: Vec<Value>,
}

pub trait GenerateAsmX86 {
    fn to_x86_32(
        &mut self,
        pos: &mut HashMap<Value, Position>,
        funcname: &mut HashMap<Function, (String, bool)>,
    ) -> String;
}

pub trait GenBlobalData {
    fn gen_global_data(&mut self, ret: &mut String, val: Value);
}

impl GenBlobalData for Program {
    fn gen_global_data(&mut self, ret: &mut String, val: Value) {
        let valkind = self.borrow_value(val).kind().clone();
        let valsize = self.borrow_value(val).ty().size() as i32;
        match valkind {
            ValueKind::Integer(intg) => {
                ret.push_str(&format!("  .word {}\n", intg.value()));
            }
            ValueKind::ZeroInit(_) => {
                ret.push_str(&format!("  .zero {}\n", valsize));
            }
            ValueKind::Aggregate(agg) => {
                for nval in agg.elems() {
                    self.gen_global_data(ret, nval.clone());
                }
            }
            _ => panic!("bad init value for global variables"),
        }
    }
}

impl GenerateAsmX86 for Program {
    fn to_x86_32(
        &mut self,
        pos: &mut HashMap<Value, Position>,
        funcname: &mut HashMap<Function, (String, bool)>,
    ) -> String {
        let mut ret = String::new();
        let mut gblvals = Vec::new();
        for &gblval in self.inst_layout() {
            gblvals.push(gblval);
        }

        let mut curi = 0;
        for gval in gblvals.iter() {
            let valkind = self.borrow_value(gval.clone()).kind().clone();
            // let valtype = self.borrow_value(gval.clone()).ty()
            match valkind {
                ValueKind::GlobalAlloc(alloc) => {
                    let myname: String = "GLBVAL_".to_owned() + &curi.to_string();
                    pos.insert(gval.clone(), Position::Global(myname.clone()));
                    ret.push_str("  .data\n");
                    ret.push_str("  .globl ");
                    ret.push_str(&myname);
                    ret.push_str("\n");
                    ret.push_str(&myname);
                    ret.push_str(":\n");
                    let initval = alloc.init();
                    self.gen_global_data(&mut ret, initval);

                    ret.push_str("\n");
                    curi += 1;
                }
                _ => {
                    continue;
                }
            }
        }

        let mut funclist: Vec<Function> = Vec::new();
        for &func in self.func_layout() {
            funclist.push(func);
        }
        for func in funclist.iter() {
            let myfunc = self.func_mut(*func);
            match myfunc.ty().kind() {
                types::TypeKind::Function(_, restype) => {
                    funcname.insert(
                        func.clone(),
                        (myfunc.name().to_string(), !restype.is_unit()),
                    );
                }
                _ => {}
            }
        }
        for func in funclist.iter() {
            let mut newpos = pos.clone();
            ret.push_str(&self.func_mut(*func).to_x86_32(&mut newpos, funcname));
            ret.push_str("\n");
        }
        return ret;
    }
}

/// 让 t\[id] = M\[sp + offset]，且只会改变 t\[id] 的值
fn gen_lw_t_sp(ret: &mut String, id: i32, offset: i32) {
    if offset < 2048 {
        ret.push_str(&format!("  lw t{}, {}(sp)\n", id, offset));
    } else {
        ret.push_str(&format!("  li t{}, {}\n", id, offset));
        ret.push_str(&format!("  add t{}, sp, t{}\n", id, id));
        ret.push_str(&format!("  lw t{}, 0(t{})\n", id, id));
    }
}

/// 让 x\[id] = M\[sp + offset]，且只会改变 x\[id] 的值
fn gen_lw_x_sp(ret: &mut String, id: i32, offset: i32) {
    if offset < 2048 {
        ret.push_str(&format!("  lw x{}, {}(sp)\n", id, offset));
    } else {
        ret.push_str(&format!("  li x{}, {}\n", id, offset));
        ret.push_str(&format!("  add x{}, sp, x{}\n", id, id));
        ret.push_str(&format!("  lw x{}, 0(x{})\n", id, id));
    }
}

fn makex(
    dfg: &DataFlowGraph,
    ret: &mut String,
    pos: &HashMap<Value, Position>,
    val: Value,
    id: i32,
) {
    // 1. 判断是否是全局变量
    let srcpos1 = pos.get(&val);
    if let Some(Position::Global(name)) = srcpos1 {
        ret.push_str(&format!("  la x{}, {}\n", id, name));
        return;
    }
    // 2. 如果是局部变量，再看数（函数中不会用到全局的数的）
    let vkind = dfg.value(val).kind().clone();
    if let ValueKind::Integer(rval) = vkind {
        ret.push_str(&format!("  li x{}, {}\n", id, rval.value()));
        return;
    }
    let srcpos = srcpos1.unwrap();
    match srcpos {
        Position::Stack(offset) => {
            gen_lw_x_sp(ret, id.clone(), offset.clone());
        }
        Position::RegX(reg) => {
            ret.push_str(&format!("  mv x{}, x{}\n", id, reg));
            return;
        }
        _ => {}
    }
    // ret.push_str(&format!("  lw x{}, 0(x{})\n", id, id));
}

fn maket(
    dfg: &DataFlowGraph,
    ret: &mut String,
    pos: &HashMap<Value, Position>,
    val: Value,
    id: i32,
) {
    if id < 3 {
        makex(dfg, ret, pos, val, id + 5);
    } else {
        makex(dfg, ret, pos, val, id + 25);
    }
}

/// 让 M\[sp + offset] = t0，会改变 t0, t1
fn store_t0_to_offset_using_t1(ret: &mut String, offset: i32) {
    if offset < 2048 {
        ret.push_str(&format!("  sw t0, {}(sp)\n", offset));
    } else {
        ret.push_str(&format!("  li t1, {}\n", offset));
        ret.push_str(&format!("  add t1, sp, t1\n"));
        ret.push_str(&format!("  sw t0, 0(t1)\n"));
    }
}

impl GenerateAsmX86 for FunctionData {
    fn to_x86_32(
        &mut self,
        pos: &mut HashMap<Value, Position>,
        funcname: &mut HashMap<Function, (String, bool)>,
    ) -> String {
        if self.layout_mut().bbs_mut().len() == 0 {
            return String::new();
        }

        let mut ret = String::new();
        ret.push_str("  .text\n");
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
                if let ValueKind::Store(_) = kind {
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
            if all_size < 2048 {
                ret.push_str(&format!("  addi sp, sp, -{}\n", all_size));
            } else {
                ret.push_str(&format!("  li t0, -{}\n", all_size));
                ret.push_str(&format!("  add sp, sp, t0\n"));
            }
        }

        // 2. 把函数参数放到合适的位置
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
        let mut curat = size_a + size_ra;
        for blockval in bbs.iter() {
            // let bb = blockval.bb.clone();
            ret.push_str(&self.name()[1..]);
            ret.push_str(&format!("_PLSDONT_{}:\n", bbids.get(&blockval.bb).unwrap()));
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
                        ret.push_str(&format!("  lw t0, 0(t0)\n"));
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
                        let sz;
                        let ptrty = self.dfg_mut().value(inst.clone()).ty().clone();
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
                        let sz;
                        let ptrty = self.dfg_mut().value(inst.clone()).ty().clone();
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
                    ValueKind::Jump(jmp) => {
                        let bb = jmp.target();
                        let id = bbids.get(&bb).unwrap();
                        ret.push_str(&format!("  j "));
                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_PLSDONT_{}\n", id));
                    }
                    ValueKind::Branch(br) => {
                        let cond = br.cond();
                        let bbtrue = br.true_bb();
                        let bbfalse = br.false_bb();
                        maket(self.dfg(), &mut ret, &pos, cond, 0);
                        ret.push_str(&format!("  beqz t0, "));
                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_PLSDONT_{}\n", bbid));

                        ret.push_str(&format!("  j "));
                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_PLSDONT_{}\n", bbids.get(&bbtrue).unwrap()));

                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_PLSDONT_{}:\n", bbid));
                        bbid += 1;
                        ret.push_str(&format!("  j "));
                        ret.push_str(&self.name()[1..]);
                        ret.push_str(&format!("_PLSDONT_{}\n", bbids.get(&bbfalse).unwrap()));
                    }
                    ValueKind::Return(re) => {
                        let val = re.value();
                        match val {
                            Some(v) => {
                                makex(self.dfg(), &mut ret, &pos, v, 10);
                                if all_size != 0 {
                                    if all_size < 2048 {
                                        ret.push_str(&format!("  addi sp, sp, {}\n", all_size));
                                    } else {
                                        ret.push_str(&format!("  li t0, {}\n", all_size));
                                        ret.push_str(&format!("  add sp, sp, t0\n"));
                                    }
                                }
                                ret.push_str(&format!("  ret\n"));
                            }
                            _ => {
                                if all_size != 0 {
                                    if all_size < 2048 {
                                        ret.push_str(&format!("  addi sp, sp, {}\n", all_size));
                                    } else {
                                        ret.push_str(&format!("  li t0, {}\n", all_size));
                                        ret.push_str(&format!("  add sp, sp, t0\n"));
                                    }
                                }
                                ret.push_str(&format!("  ret\n"));
                            }
                        }
                    }
                    ValueKind::Binary(bin) => {
                        let lhs = bin.lhs();
                        let rhs = bin.rhs();
                        maket(self.dfg(), &mut ret, &pos, lhs, 0);
                        maket(self.dfg(), &mut ret, &pos, rhs, 1);
                        match bin.op() {
                            BinaryOp::NotEq => {
                                ret.push_str(&format!("  sub t1, t0, t1\n"));
                                ret.push_str(&format!("  snez t0, t1\n"));
                            }
                            BinaryOp::Eq => {
                                ret.push_str(&format!("  sub t1, t0, t1\n"));
                                ret.push_str(&format!("  seqz t0, t1\n"));
                            }
                            BinaryOp::Gt => {
                                ret.push_str(&format!("  sgt t0, t0, t1\n"));
                            }
                            BinaryOp::Lt => {
                                ret.push_str(&format!("  slt t0, t0, t1\n"));
                            }
                            BinaryOp::Ge => {
                                ret.push_str(&format!("  slt t0, t0, t1\n"));
                                ret.push_str(&format!("  xori t0, t0, 1\n"));
                            }
                            BinaryOp::Le => {
                                ret.push_str(&format!("  sgt t0, t0, t1\n"));
                                ret.push_str(&format!("  xori t0, t0, 1\n"));
                            }
                            BinaryOp::Add => {
                                ret.push_str(&format!("  add t0, t0, t1\n"));
                            }
                            BinaryOp::Sub => {
                                ret.push_str(&format!("  sub t0, t0, t1\n"));
                            }
                            BinaryOp::Mul => {
                                ret.push_str(&format!("  mul t0, t0, t1\n"));
                            }
                            BinaryOp::Div => {
                                ret.push_str(&format!("  div t0, t0, t1\n"));
                            }
                            BinaryOp::Mod => {
                                ret.push_str(&format!("  rem t0, t0, t1\n"));
                            }
                            // bitwise and or xor
                            BinaryOp::And => {
                                ret.push_str(&format!("  and t0, t0, t1\n"));
                            }
                            BinaryOp::Or => {
                                ret.push_str(&format!("  or t0, t0, t1\n"));
                            }
                            BinaryOp::Xor => {
                                ret.push_str(&format!("  xor t0, t0, t1\n"));
                            }
                            _ => {
                                panic!("not implemented")
                            }
                        }
                        store_t0_to_offset_using_t1(&mut ret, curat);
                        pos.insert(inst, Position::Stack(curat));
                        curat += 4;
                    }
                    ValueKind::Call(call) => {
                        let mut i = 0;
                        for arg in call.args() {
                            if i < 8 {
                                makex(self.dfg(), &mut ret, &pos, arg.clone(), 10 + i);
                            } else {
                                maket(self.dfg(), &mut ret, &pos, arg.clone(), 0);
                                store_t0_to_offset_using_t1(&mut ret, 4 * (i - 8));
                            }
                            i += 1;
                        }
                        ret.push_str(&format!("  mv t0, ra\n"));
                        store_t0_to_offset_using_t1(&mut ret, size_a);

                        let funcdata = funcname.get(&call.callee()).unwrap();

                        ret.push_str(&format!("  call "));
                        ret.push_str(&funcdata.0[1..]);
                        ret.push_str("\n");

                        gen_lw_t_sp(&mut ret, 0, size_a);
                        ret.push_str(&format!("  mv ra, t0\n")); // reload ra

                        if funcdata.1 {
                            ret.push_str(&format!("  mv t0, a0\n"));
                            store_t0_to_offset_using_t1(&mut ret, curat);
                            pos.insert(inst, Position::Stack(curat));
                            curat += 4;
                        }
                    }
                    _ => {
                        panic!("bad value for instruction");
                    }
                }
            }
        }
        return ret;
    }
}
