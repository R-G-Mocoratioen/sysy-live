#[derive(Debug, Clone)]
pub struct CompUnit {
    pub func_defs: Vec<FuncDef>,
    pub global_var_defs: Vec<Decl>,
}

#[derive(Debug, Clone)]
pub enum GlobalDef {
    FuncDef(FuncDef),
    Decl(Decl),
}

#[derive(Debug, Clone)]
pub enum FuncParam {
    Var(String),
    Array(String, Vec<Box<Exp>>),
}
// 目前只有 int 一个类型。

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub params: Vec<FuncParam>,
    pub id: String,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub enum FuncType {
    Int,
    Void,
}

#[derive(Debug, Clone)]
pub struct Block {
    // pub stmt: Stmt,
    pub vecitem: Vec<Box<BlockItem>>,
}

#[derive(Debug, Clone)]
pub enum BlockItem {
    Stmt(Box<Stmt>),
    Decl(Box<Decl>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(LVal, Box<Exp>),
    Return(Box<Option<Exp>>),
    Do(Box<Option<Exp>>),
    Block(Box<Block>),
    If(Box<Exp>, Box<Stmt>),
    IfElse(Box<Exp>, Box<Stmt>, Box<Stmt>),
    While(Box<Exp>, Box<Stmt>),
    Break,
    Continue,
    Sing(Box<Exp>, Box<Exp>, Box<Exp>, Box<Exp>, Box<Exp>, Box<Exp>),
    PushBar(Box<Exp>, Box<Exp>),
    PushNote(Box<Exp>, Box<Exp>),
    SetBarBpm(Box<Exp>, Box<Exp>),
    SetScoreBpm(Box<Exp>, Box<Exp>),
    Append(Box<Exp>, Box<Exp>),
    ReplaceBar(Box<Exp>, Box<Exp>, Box<Exp>),
    IncScorePitch(Box<Exp>, Box<Exp>),
    IncBarPitch(Box<Exp>, Box<Exp>),
    SetScoreDuration(Box<Exp>, Box<Exp>),
    SetBarDuration(Box<Exp>, Box<Exp>),
}

#[derive(Debug, Clone)]
pub struct Exp {
    pub lorexp: Box<LOrExp>,
}

#[derive(Debug, Clone)]
pub enum LVal {
    Ident(String),
    Array(String, Vec<Box<Exp>>),
}

#[derive(Debug, Clone)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    LVal(LVal),
    Number(i32),
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub defs: Vec<Box<VarDef>>,
}

#[derive(Debug, Clone)]
pub enum ArrayInit {
    Single(Box<Exp>),
    Multiple(Vec<Box<ArrayInit>>),
}

#[derive(Debug, Clone)]
pub enum VarDef {
    Ident(String),
    Array(String, Vec<Box<Exp>>),
    ArrayInit(String, Vec<Box<Exp>>, Box<ArrayInit>),
    IdentInit(String, Box<Exp>),
    ConstIdentInit(String, Box<Exp>),
    ScoreInit(String, Box<Score>),
    BarInit(String, Box<Bar>),
    NoteInit(String, Box<Note>, i32, i32),
    BarCopy(String, Box<Exp>),
    Bar(String),
    Score(String),
    ScoreCopy(String, Box<Exp>),
}

#[derive(Debug, Clone)]
pub enum UnaryExp {
    PrimaryExp(Box<PrimaryExp>),
    FuncCall(String, Vec<Box<Exp>>),
    Pos(Box<UnaryExp>),
    Neg(Box<UnaryExp>),
    Not(Box<UnaryExp>),
}

#[derive(Debug, Clone)]
pub enum MulExp {
    UnaryExp(Box<UnaryExp>),
    Mul(Box<MulExp>, Box<UnaryExp>),
    Div(Box<MulExp>, Box<UnaryExp>),
    Mod(Box<MulExp>, Box<UnaryExp>),
}

#[derive(Debug, Clone)]
pub enum AddExp {
    MulExp(Box<MulExp>),
    Add(Box<AddExp>, Box<MulExp>),
    Sub(Box<AddExp>, Box<MulExp>),
}

#[derive(Debug, Clone)]
pub enum RelExp {
    AddExp(Box<AddExp>),
    Lt(Box<RelExp>, Box<AddExp>),
    Le(Box<RelExp>, Box<AddExp>),
    Gt(Box<RelExp>, Box<AddExp>),
    Ge(Box<RelExp>, Box<AddExp>),
}

#[derive(Debug, Clone)]
pub enum EqExp {
    RelExp(Box<RelExp>),
    Eq(Box<EqExp>, Box<RelExp>),
    Ne(Box<EqExp>, Box<RelExp>),
}

#[derive(Debug, Clone)]
pub enum LAndExp {
    EqExp(Box<EqExp>),
    And(Box<LAndExp>, Box<EqExp>),
}

#[derive(Debug, Clone)]
pub enum LOrExp {
    LAndExp(Box<LAndExp>),
    Or(Box<LOrExp>, Box<LAndExp>),
}

#[derive(Debug, Clone)]
pub enum Note {
    Semitone(i32),
    Rest,
}

impl Note {
    pub fn up(self, num: i32) -> Note {
        match self {
            Note::Semitone(n) => Note::Semitone(n + num * 12),
            Note::Rest => Note::Rest,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NoteElem {
    Tie,
    Note(Note),
    Notes(Vec<Box<NoteElem>>),
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub notes: Vec<NoteElem>,
}

#[derive(Debug, Clone)]
pub struct Score {
    pub bars: Vec<Bar>,
}
