use super::{types::Type, Ident};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'ast> {
    LiteralString(&'ast str),
    LiteralChar(char),

    LiteralShort(i16),
    LiteralInt(i32),
    LiteralLong(i64),
    LiteralFloat(f32),
    LiteralDouble(f64),

    Ident(Ident<'ast>),

    Prefix(PrefixExpr<'ast>),
    Infix(InfixExpr<'ast>),
    Post(PostExpr<'ast>),

    Call(CallExpr<'ast>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr<'ast> {
    pub val: &'ast Expression<'ast>,
    pub args: Vec<Expression<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpr<'ast> {
    pub val: &'ast Expression<'ast>,
    pub op: PreOperator<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpr<'ast> {
    pub left: &'ast Expression<'ast>,
    pub right: &'ast Expression<'ast>,
    pub op: InOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PostExpr<'ast> {
    pub val: &'ast Expression<'ast>,
    pub op: PostOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PreOperator<'ast> {
    Pos,
    Neg,
    Not,
    BNot,
    Deref,
    SizeOf,
    AddrOf,
    AlignOf,
    Cast(Type<'ast>),
    Incr,
    Decr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InOperator {
    // Arithmetic Operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Bitwise Operators
    LSh,
    RSh,
    BAnd,
    BOr,
    BXor,
    // Relational Operators
    Eq,
    Neq,
    LT,
    GT,
    LTE,
    GTE,
    // Logical Operators
    And,
    Or,
    // Assignment
    Assign,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignLsh,
    AssignRsh,
    AssingBAnd,
    AssignBOr,
    AssignBXor,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostOperator {
    Incr,
    Decr,
}
