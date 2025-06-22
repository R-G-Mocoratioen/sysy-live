use crate::riscv::*;
use koopa::ir::builder::*;
use koopa::ir::dfg::DataFlowGraph;
use koopa::ir::*;
use std::collections::HashMap;
pub trait OptimizeExp {
    fn optimize_exp(&mut self) -> bool;
}

impl OptimizeExp for Program {
    fn optimize_exp(&mut self) -> bool {
        let mut funclist: Vec<Function> = Vec::new();
        for &func in self.func_layout() {
            funclist.push(func);
        }
        let mut res = false;
        for func in funclist.iter() {
            res |= self.func_mut(*func).optimize_exp();
        }
        return res;
    }
}

fn remove_conflicting_value(dfg: &DataFlowGraph, pre_vals: Vec<Value>, inst: Value) -> Vec<Value> {
    let instkind = dfg.value(inst.clone()).kind().clone();
    match instkind {
        ValueKind::Store(str) => {
            let mut new_vec: Vec<Value> = Vec::new();
            for val in pre_vals.iter() {
                match dfg.value(val.clone()).kind().clone() {
                    ValueKind::Load(ld) => {
                        if str.dest() == ld.src() {
                            continue;
                        }
                        new_vec.push(val.clone());
                    }
                    ValueKind::Store(st) => {
                        if str.dest() == st.dest() {
                            continue;
                        }
                        new_vec.push(val.clone());
                    }
                    _ => {
                        new_vec.push(val.clone());
                    }
                }
            }
            return new_vec;
        }
        _ => {
            return pre_vals;
        }
    }
}

impl OptimizeExp for FunctionData {
    fn optimize_exp(&mut self) -> bool {
        let mut bbids: Vec<BasicBlock> = Vec::new();
        let mut bbs: Vec<BasicBlockVal> = Vec::new();
        let mut paramlist: HashMap<Value, bool> = HashMap::new();
        for (&bb, _) in self.layout_mut().bbs_mut().iter() {
            bbids.push(bb);
        }

        let mut prev_bbs: HashMap<BasicBlock, Vec<BasicBlock>> = HashMap::new();
        for &bb in bbids.iter() {
            let mut t = BasicBlockVal {
                bb: bb.clone(),
                insts: Vec::new(),
            };

            for (&inst, _) in self.layout_mut().bb_mut(bb).insts_mut().iter() {
                t.insts.push(inst.clone());
                match self.dfg().value(inst.clone()).kind().clone() {
                    ValueKind::Branch(br) => {
                        let bbf = br.false_bb();
                        let bbt = br.true_bb();
                        if !prev_bbs.contains_key(&bbf) {
                            prev_bbs.insert(bbf.clone(), Vec::new());
                        }
                        if !prev_bbs.contains_key(&bbt) {
                            prev_bbs.insert(bbt.clone(), Vec::new());
                        }
                        prev_bbs.get_mut(&bbf).unwrap().push(bb.clone());
                        prev_bbs.get_mut(&bbt).unwrap().push(bb.clone());
                    }
                    ValueKind::Jump(jr) => {
                        let bbt = jr.target();
                        if !prev_bbs.contains_key(&bbt) {
                            prev_bbs.insert(bbt.clone(), Vec::new());
                        }
                        prev_bbs.get_mut(&bbt).unwrap().push(bb.clone());
                    }
                    _ => {}
                }
            }
            bbs.push(t);
        }

        for par in self.params() {
            paramlist.insert(par.clone(), true);
        }
        let mut replacing: Vec<(Value, BasicBlock, Value)> = Vec::new();

        let mut pre_vals: HashMap<Value, Vec<Value>> = HashMap::new();
        let mut bb_last_vals: HashMap<BasicBlock, Vec<Value>> = HashMap::new();
        loop {
            let mut changed = false;
            for blockval in bbs.iter() {
                if pre_vals.get(&blockval.insts[0]).is_none() {
                    pre_vals.insert(blockval.insts[0].clone(), Vec::new());
                    changed = true;
                }
                let mut first_pre = pre_vals
                    .get(&blockval.insts[0])
                    .expect("should be inited")
                    .clone();
                let mut ind: usize = 0;
                for inst in blockval.insts.iter() {
                    let mut new_vec =
                        remove_conflicting_value(self.dfg(), first_pre.clone(), inst.clone());
                    let match_my = get_matching_value(self.dfg(), new_vec.clone(), inst.clone());
                    if match_my.is_some() {
                        let my = match_my.unwrap();
                        assert!(my != inst.clone());
                        add_to_replacing(&mut replacing, inst.clone(), blockval.bb, my);
                    } else {
                        match self.dfg().value(inst.clone()).kind().clone() {
                            ValueKind::Binary(bin) => {
                                new_vec.push(inst.clone());
                            }
                            ValueKind::GetElemPtr(va) => {
                                new_vec.push(inst.clone());
                            }
                            ValueKind::GetPtr(va) => {
                                new_vec.push(inst.clone());
                            }
                            ValueKind::Load(_) => {
                                new_vec.push(inst.clone());
                            }
                            ValueKind::Store(st) => {
                                new_vec.push(inst.clone());
                            }
                            ValueKind::Branch(br) => {
                                let tmp = bb_last_vals.get(&blockval.bb);
                                if tmp.is_none() {
                                    bb_last_vals.insert(blockval.bb, new_vec);
                                    changed = true;
                                    break;
                                } else {
                                    let last = tmp.unwrap();
                                    if *last != new_vec {
                                        bb_last_vals.insert(blockval.bb, new_vec);
                                        changed = true;
                                    }
                                    break;
                                }
                            }
                            ValueKind::Call(ca) => {
                                let tmp = bb_last_vals.get(&blockval.bb);
                                if tmp.is_none() {
                                    bb_last_vals.insert(blockval.bb, new_vec);
                                    changed = true;
                                    break;
                                } else {
                                    let last = tmp.unwrap();
                                    if *last != new_vec {
                                        bb_last_vals.insert(blockval.bb, new_vec);
                                        changed = true;
                                    }
                                    break;
                                }
                            }
                            ValueKind::Return(_) => {
                                let tmp = bb_last_vals.get(&blockval.bb);
                                if tmp.is_none() {
                                    bb_last_vals.insert(blockval.bb, new_vec);
                                    changed = true;
                                    break;
                                } else {
                                    let last = tmp.unwrap();
                                    if *last != new_vec {
                                        bb_last_vals.insert(blockval.bb, new_vec);
                                        changed = true;
                                    }
                                    break;
                                }
                            }
                            _ => {
                                unreachable!("unexpected kind");
                            }
                        }
                    }
                    first_pre = new_vec;
                    let tmp = pre_vals.get(&inst);
                    if tmp.is_none() {
                        pre_vals.insert(inst.clone(), first_pre.clone());
                    } else if *tmp.unwrap() != first_pre {
                        pre_vals.insert(inst.clone(), first_pre.clone());
                    }
                }
                match self
                    .dfg()
                    .value(blockval.insts.last().unwrap().clone())
                    .kind()
                    .clone()
                {
                    ValueKind::Branch(br) => {
                        changed |= update_bb_first(&mut bb_last_vals, br.true_bb().clone());
                        changed |= update_bb_first(&mut bb_last_vals, br.false_bb().clone());
                    }
                    ValueKind::Jump(jr) => {
                        changed |= update_bb_first(&mut bb_last_vals, jr.target().clone());
                    }
                    ValueKind::Return(ret) => {}
                    _ => panic!("unexpected kind at end of block"),
                }
            }
            if changed == false {
                break;
            }
        }

        if replacing.len() == 0 {
            return false;
        }

        for (inst, bb, replacer) in replacing.iter().rev() {
            let usedby = self.dfg().value(inst.clone()).used_by().clone();
            for user in &usedby {
                match self.dfg().value(user.clone()).kind().clone() {
                    ValueKind::Binary(bin) => {
                        let op = bin.op();
                        let mut lhs = bin.lhs();
                        let mut rhs = bin.rhs();
                        if lhs == inst.clone() {
                            lhs = replacer.clone();
                        }
                        if rhs == inst.clone() {
                            rhs = replacer.clone();
                        }
                        self.dfg_mut()
                            .replace_value_with(user.clone())
                            .binary(op, lhs, rhs);
                    }
                    ValueKind::Branch(br) => {
                        let bbf = br.false_bb();
                        let bbt = br.true_bb();
                        self.dfg_mut().replace_value_with(user.clone()).branch(
                            replacer.clone(),
                            bbt,
                            bbf,
                        );
                    }
                    ValueKind::Call(ca) => {
                        let args = ca.args();
                        let callee = ca.callee();
                        let vec = args
                            .iter()
                            .map(|v| {
                                if v.clone() == inst.clone() {
                                    replacer.clone()
                                } else {
                                    v.clone()
                                }
                            })
                            .collect();
                        self.dfg_mut()
                            .replace_value_with(user.clone())
                            .call(callee, vec);
                        // panic!("exit here");
                    }
                    ValueKind::GetElemPtr(va) => {
                        let mut lhs = va.src();
                        let mut rhs = va.index();
                        if lhs == inst.clone() {
                            lhs = replacer.clone();
                        }
                        if rhs == inst.clone() {
                            rhs = replacer.clone();
                        }
                        self.dfg_mut()
                            .replace_value_with(user.clone())
                            .get_elem_ptr(lhs, rhs);
                    }
                    ValueKind::GetPtr(va) => {
                        let mut lhs = va.src();
                        let mut rhs = va.index();
                        if lhs == inst.clone() {
                            lhs = replacer.clone();
                        }
                        if rhs == inst.clone() {
                            rhs = replacer.clone();
                        }
                        self.dfg_mut()
                            .replace_value_with(user.clone())
                            .get_ptr(lhs, rhs);
                    }
                    ValueKind::Load(_) => {
                        self.dfg_mut()
                            .replace_value_with(user.clone())
                            .load(replacer.clone());
                    }
                    ValueKind::Return(_) => {
                        self.dfg_mut()
                            .replace_value_with(user.clone())
                            .ret(Some(replacer.clone()));
                    }
                    ValueKind::Store(st) => {
                        let mut lhs = st.value();
                        let mut rhs = st.dest();
                        if lhs == inst.clone() {
                            lhs = replacer.clone();
                        }
                        if rhs == inst.clone() {
                            rhs = replacer.clone();
                        }
                        self.dfg_mut()
                            .replace_value_with(user.clone())
                            .store(lhs, rhs);
                    }
                    _ => {
                        unreachable!("unexpected kind");
                    }
                }
            }
            let len = self.dfg().value(inst.clone()).used_by().len();
            if len == 0 {
                self.layout_mut()
                    .bb_mut(bb.clone())
                    .insts_mut()
                    .remove(inst);
                let _ = self.dfg_mut().remove_value(inst.clone());
            }
        }
        return true;
    }
}
