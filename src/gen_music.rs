use crate::ast::*;
use crate::ident::*;
use koopa::ir::builder::LocalInstBuilder;
use koopa::ir::builder::ValueBuilder;
use koopa::ir::*;
use std::collections::HashMap;

pub fn call_func(
    name: &str,
    data: &mut FunctionData,
    entry: &mut BasicBlock,
    var: &mut HashMap<String, IdentValue>,
    args: Vec<Value>,
) -> Value {
    let func = var.get(name).unwrap();
    match func {
        IdentValue::Func(func) => {
            let call = data.dfg_mut().new_value().call(func.clone(), args);
            data.layout_mut().bb_mut(*entry).insts_mut().extend([call]);
            return call;
        }
        _ => panic!("{} is not a function", name),
    }
}

pub fn gen_note(
    data: &mut FunctionData,
    entry: &mut BasicBlock,
    var: &mut HashMap<String, IdentValue>,
    note: Note,
    fz: i32,
    fm: i32,
) -> Value {
    let ffz = data.dfg_mut().new_value().integer(fz);
    let ffm = data.dfg_mut().new_value().integer(fm);
    let one = data.dfg_mut().new_value().integer(1);
    match note {
        Note::Rest => {
            let it = call_func("newnote_rest", data, entry, var, vec![one, ffz, ffm]);
            return it;
        }
        Note::Semitone(num) => {
            let num = data.dfg_mut().new_value().integer(num.clone());
            let it = call_func("newnote", data, entry, var, vec![num, ffz, ffm]);
            return it;
        }
    }
}

pub fn gen_note_in_bar(
    data: &mut FunctionData,
    entry: &mut BasicBlock,
    var: &mut HashMap<String, IdentValue>,
    note: &NoteElem,
    bar: Value,
    fz: i32,
    fm: i32,
) {
    let ffz = data.dfg_mut().new_value().integer(fz);
    let ffm = data.dfg_mut().new_value().integer(fm);
    let one = data.dfg_mut().new_value().integer(1);
    let two = data.dfg_mut().new_value().integer(2);
    match note {
        NoteElem::Tie => {
            let it = call_func("newnote_rest", data, entry, var, vec![two, ffz, ffm]);
            call_func("bar_push", data, entry, var, vec![bar, it]);
        }
        NoteElem::Note(Note::Rest) => {
            let it = call_func("newnote_rest", data, entry, var, vec![one, ffz, ffm]);
            call_func("bar_push", data, entry, var, vec![bar, it]);
        }
        NoteElem::Note(Note::Semitone(num)) => {
            let num = data.dfg_mut().new_value().integer(num.clone());
            let it = call_func("newnote", data, entry, var, vec![num, ffz, ffm]);
            call_func("bar_push", data, entry, var, vec![bar, it]);
        }
        NoteElem::Notes(notes) => {
            for note in notes.iter() {
                gen_note_in_bar(data, entry, var, note, bar, fz, fm * (notes.len() as i32));
            }
        }
    }
}

pub fn gen_bar(
    data: &mut FunctionData,
    entry: &mut BasicBlock,
    var: &mut HashMap<String, IdentValue>,
    bar: &Bar,
) -> Value {
    let it = call_func("newbar", data, entry, var, vec![]);
    for note in bar.notes.iter() {
        gen_note_in_bar(data, entry, var, note, it, 1, 1);
        // rest, tie 的处理让 imp.cpp 来完成。
    }
    return it;
}

pub fn gen_score(
    data: &mut FunctionData,
    entry: &mut BasicBlock,
    var: &mut HashMap<String, IdentValue>,
    score: &Score,
) -> Value {
    let it = call_func("newscore", data, entry, var, vec![]);
    for note in score.bars.iter() {
        let it2 = gen_bar(data, entry, var, note);
        call_func("score_push", data, entry, var, vec![it, it2]);
        // bar 内自己处理
        // bar 间 imp.cpp 处理
    }
    return it;
}

pub fn count_semitone(x: i32) -> i32 {
    if x == 1 {
        return 0;
    }
    if x == 2 {
        return 2;
    }
    if x == 3 {
        return 4;
    }
    if x == 4 {
        return 5;
    }
    if x == 5 {
        return 7;
    }
    if x == 6 {
        return 9;
    }
    if x == 7 {
        return 11;
    }
    unreachable!();
}
