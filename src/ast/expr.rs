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

    Deref(&'ast Expression<'ast>),
    AddrOf(&'ast Expression<'ast>),

    Sizeof(&'ast Expression<'ast>),

    Call(CallExpr<'ast>),
    BinaryOperation(BinOpExpr<'ast>),
    UnaryOperation(UnOpExpr<'ast>),

    Prefix(PrefixExpr<'ast>),

    // Assignment
    CompoundAssignment(CompoundAssignmentExpr<'ast>),
    Assignment(AssignmentExpr<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpr<'ast> {
    pub val: &'ast Expression<'ast>,
    pub op: PreOperator<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr<'ast> {
    pub name: Ident<'ast>,
    pub args: Vec<Expression<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinOpExpr<'ast> {
    pub left: &'ast Expression<'ast>,
    pub right: &'ast Expression<'ast>,
    pub operator: BinOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnOpExpr<'ast> {
    pub left: &'ast Expression<'ast>,
    pub operator: UnOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompoundAssignmentExpr<'ast> {
    pub ident: Ident<'ast>,
    pub val: &'ast Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpr<'ast> {
    pub ident: Ident<'ast>,
    pub val: &'ast Expression<'ast>,
    pub operator: BinOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LSh,
    RSh,
    BAnd,
    BOr,
    BXor,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnOperator {
    Incr,
    Decr,
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
}
