#[derive(Debug)]
pub struct CompUnit {
    pub func_defs: Vec<FuncDef>,
    pub global_var_defs: Vec<Decl>,
}

#[derive(Debug)]
pub enum GlobalDef {
    FuncDef(FuncDef),
    Decl(Decl),
}

#[derive(Debug)]
pub struct FuncParam {
    pub id: String,
}
// 目前只有 int 一个类型。

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub params: Vec<FuncParam>,
    pub id: String,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Int,
    Void,
}

#[derive(Debug)]
pub struct Block {
    // pub stmt: Stmt,
    pub vecitem: Vec<Box<BlockItem>>,
}

#[derive(Debug)]
pub enum BlockItem {
    Stmt(Box<Stmt>),
    Decl(Box<Decl>),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Box<Exp>),
    Return(Box<Option<Exp>>),
    Do(Box<Option<Exp>>),
    Block(Box<Block>),
    If(Box<Exp>, Box<Stmt>),
    IfElse(Box<Exp>, Box<Stmt>, Box<Stmt>),
    While(Box<Exp>, Box<Stmt>),
    Break,
    Continue,
}

#[derive(Debug)]
pub struct Exp {
    pub lorexp: Box<LOrExp>,
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    LVal(String),
    Number(i32),
}

#[derive(Debug)]
pub struct Decl {
    pub defs: Vec<Box<VarDef>>,
}

#[derive(Debug)]
pub enum VarDef {
    Ident(String),
    IdentInit(String, Box<Exp>),
    ConstIdentInit(String, Box<Exp>),
}

#[derive(Debug)]
pub enum UnaryExp {
    PrimaryExp(Box<PrimaryExp>),
    FuncCall(String, Vec<Box<Exp>>),
    Pos(Box<UnaryExp>),
    Neg(Box<UnaryExp>),
    Not(Box<UnaryExp>),
}

#[derive(Debug)]
pub enum MulExp {
    UnaryExp(Box<UnaryExp>),
    Mul(Box<MulExp>, Box<UnaryExp>),
    Div(Box<MulExp>, Box<UnaryExp>),
    Mod(Box<MulExp>, Box<UnaryExp>),
}

#[derive(Debug)]
pub enum AddExp {
    MulExp(Box<MulExp>),
    Add(Box<AddExp>, Box<MulExp>),
    Sub(Box<AddExp>, Box<MulExp>),
}

#[derive(Debug)]
pub enum RelExp {
    AddExp(Box<AddExp>),
    Lt(Box<RelExp>, Box<AddExp>),
    Le(Box<RelExp>, Box<AddExp>),
    Gt(Box<RelExp>, Box<AddExp>),
    Ge(Box<RelExp>, Box<AddExp>),
}

#[derive(Debug)]
pub enum EqExp {
    RelExp(Box<RelExp>),
    Eq(Box<EqExp>, Box<RelExp>),
    Ne(Box<EqExp>, Box<RelExp>),
}

#[derive(Debug)]
pub enum LAndExp {
    EqExp(Box<EqExp>),
    And(Box<LAndExp>, Box<EqExp>),
}

#[derive(Debug)]
pub enum LOrExp {
    LAndExp(Box<LAndExp>),
    Or(Box<LOrExp>, Box<LAndExp>),
}
