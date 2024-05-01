pub type Ident<'ast> = &'ast str;

#[derive(Debug, PartialEq, Clone)]
pub enum Type<'ast> {
    Struct(StructStmt<'ast>),
    Enum(EnumStmt<'ast>),
    Union(UnionStmt<'ast>),
    Auto,
    Ident(&'ast str),
    Pointer(&'ast Type<'ast>),
}

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
pub struct StructStmt<'ast> {
    pub name: Option<Ident<'ast>>,
    pub fields: Vec<Field<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field<'ast> {
    pub name: Ident<'ast>,
    pub _type: Type<'ast>,
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
    pub is_static: bool,
    pub is_inline: bool,
    pub args: Vec<Field<'ast>>,
    pub ret_type: Type<'ast>,
    pub body: Option<BlockStmt<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableStmt<'ast> {
    pub name: Ident<'ast>,
    pub is_volatile: bool,
    pub is_const: bool,
    pub is_static: bool,
    pub is_register: bool,
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
