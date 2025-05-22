use crate::riscv::*;
use koopa::ir::builder::*;
use koopa::ir::dfg::*;
use koopa::ir::*;
use std::collections::HashMap;
pub trait OptimizeLoadStore {
    fn optimize_loadstore(&mut self);
}

impl OptimizeLoadStore for Program {
    fn optimize_loadstore(&mut self) {
        let mut funclist: Vec<Function> = Vec::new();
        for &func in self.func_layout() {
            funclist.push(func);
        }
        for func in funclist.iter() {
            self.func_mut(*func).optimize_loadstore();
        }
    }
}

impl OptimizeLoadStore for FunctionData {
    fn optimize_loadstore(&mut self) {
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
            let mut stored: HashMap<Value, Value> = HashMap::new();
            for inst in blockval.insts.iter() {
                let kind = self.dfg_mut().value(inst.clone()).kind().clone();
                match kind {
                    ValueKind::Store(store) => {
                        let src = store.value();
                        let dst = store.dest();
                        stored.insert(dst.clone(), src.clone()); // stored[dst] = src
                    }
                    ValueKind::Load(load) => {
                        let src = load.src();
                        let res = stored.get(&src);
                        if !res.is_none() {
                            let replacer = res.unwrap();
                            /*let replacerdata = self.dfg_mut().value(replacer.clone());
                            self.dfg_mut()
                                .replace_value_with(inst.clone())
                                .insert_value(replacerdata);*/
                            let usedby = self.dfg().value(inst.clone()).used_by().clone();
                            for user in &usedby {
                                match self.dfg().value(user.clone()).kind().clone() {
                                    ValueKind::Binary(bin) => {
                                        let op = bin.op();
                                        let mut lhs = bin.lhs();
                                        let mut rhs = bin.rhs();
                                        if self.dfg().value_eq(lhs, inst.clone()) {
                                            lhs = replacer.clone();
                                        }
                                        if self.dfg().value_eq(rhs, inst.clone()) {
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
                                                if self.dfg().value_eq(v.clone(), inst.clone()) {
                                                    replacer.clone()
                                                } else {
                                                    v.clone()
                                                }
                                            })
                                            .collect();
                                        self.dfg_mut()
                                            .replace_value_with(user.clone())
                                            .call(callee, vec);
                                    }
                                    ValueKind::GetElemPtr(va) => {
                                        let mut lhs = va.src();
                                        let mut rhs = va.index();
                                        if self.dfg().value_eq(lhs, inst.clone()) {
                                            lhs = replacer.clone();
                                        }
                                        if self.dfg().value_eq(rhs, inst.clone()) {
                                            rhs = replacer.clone();
                                        }
                                        self.dfg_mut()
                                            .replace_value_with(user.clone())
                                            .get_elem_ptr(lhs, rhs);
                                    }
                                    ValueKind::GetPtr(va) => {
                                        let mut lhs = va.src();
                                        let mut rhs = va.index();
                                        if self.dfg().value_eq(lhs, inst.clone()) {
                                            lhs = replacer.clone();
                                        }
                                        if self.dfg().value_eq(rhs, inst.clone()) {
                                            rhs = replacer.clone();
                                        }
                                        self.dfg_mut()
                                            .replace_value_with(user.clone())
                                            .get_elem_ptr(lhs, rhs);
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
                                        if self.dfg().value_eq(lhs, inst.clone()) {
                                            lhs = replacer.clone();
                                        }
                                        if self.dfg().value_eq(rhs, inst.clone()) {
                                            rhs = replacer.clone();
                                        }
                                        self.dfg_mut()
                                            .replace_value_with(user.clone())
                                            .store(lhs, rhs);
                                    }
                                    _ => {}
                                }
                            }
                            let len = self.dfg().value(inst.clone()).used_by().len();
                            if len == 0 {
                                self.layout_mut()
                                    .bb_mut(blockval.bb)
                                    .insts_mut()
                                    .remove(inst);
                                let _ = self.dfg_mut().remove_value(inst.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
