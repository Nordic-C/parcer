use std::fmt::Display;

use super::{types::Type, expr::Expression, Ident};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'ast> {
    // Data types
    Struct(StructStmt<'ast>),
    Enum(EnumStmt<'ast>),
    Union(UnionStmt<'ast>),

    Label(LabelStmt<'ast>),

    Function(FunctionStmt<'ast>),
    Variable(VariableStmt<'ast>),

    // Control flow
    If(IfStmt<'ast>),
    Switch(SwitchStmt<'ast>),

    // Loops
    While(WhileStmt<'ast>),
    DoWhile(DoWhileStmt<'ast>),
    For(ForStmt),

    Typedef(TypedefStmt<'ast>),

    Return(ReturnStmt<'ast>),
    Break(BreakStmt<'ast>),
    Continue(ContinueStmt<'ast>),
    Goto(GotoStmt<'ast>),

    Block(BlockStmt<'ast>),

    Expression(Expression<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructStmt<'ast> {
    pub name: Option<Ident<'ast>>,
    pub fields: Vec<Field<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field<'ast> {
    pub name: Ident<'ast>,
    pub field_type: Type<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumStmt<'ast> {
    pub name: Option<Ident<'ast>>,
    pub variants: Vec<Ident<'static>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnionStmt<'ast> {
    pub name: Option<Ident<'ast>>,
    pub fields: Vec<Field<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionStmt<'ast> {
    pub name: Ident<'ast>,
    pub is_volatile: bool,
    pub should_inline: bool,
    pub data_storage_class: DataStorageClass,
    pub args: Vec<Field<'ast>>,
    pub ret_type: Type<'ast>,
    pub body: Option<BlockStmt<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableStmt<'ast> {
    pub name: Ident<'ast>,
    pub is_volatile: bool,
    pub is_const: bool,
    pub data_storage_class: DataStorageClass,
    pub _type: Type<'ast>,
    pub val: Option<Expression<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IfType {
    If,
    ElseIf,
    Else,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt<'ast> {
    pub if_type: IfType,
    pub cond: Option<Expression<'ast>>,
    pub block: BlockStmt<'ast>,
    pub alt: Option<&'ast IfStmt<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStmt<'ast> {
    pub comp_val: Expression<'ast>,
    pub cases: Vec<CaseStmt<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseStmt<'ast> {
    pub comp_val: Expression<'ast>,
    pub block: BlockStmt<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStmt<'ast> {
    pub cond: Expression<'ast>,
    pub block: BlockStmt<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DoWhileStmt<'ast> {
    pub cond: Expression<'ast>,
    pub block: BlockStmt<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStmt {}

#[derive(Debug, PartialEq, Clone)]
pub struct TypedefStmt<'ast> {
    pub name: Ident<'ast>,
    pub _type: &'ast Statement<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt<'ast> {
    pub val: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStmt<'ast> {
    pub label: Option<Ident<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContinueStmt<'ast> {
    pub label: Option<Ident<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GotoStmt<'ast> {
    pub label: Option<Ident<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStmt<'ast> {
    pub block: Vec<Statement<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LabelStmt<'ast> {
    pub name: Ident<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataStorageClass {
    Static,
    Extern,
    Register,
    Auto,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CompositeDataType {
    Struct,
    Union,
    Enum,
}

impl Display for CompositeDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CompositeDataType::Struct => "struct",
            CompositeDataType::Union => "union",
            CompositeDataType::Enum => "enum",
        })
    }
}