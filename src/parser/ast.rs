pub type Ident<'a> = &'a str;

#[derive(Debug, PartialEq, Clone)]
pub enum Type<'a> {
    Struct(StructStmt<'a>),
    Enum(EnumStmt<'a>),
    Union(UnionStmt<'a>),
    Auto,
    Ident(&'a str)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'a> {
    // Data types
    Struct(StructStmt<'a>),
    Enum(EnumStmt<'a>),
    Union(UnionStmt<'a>),

    Function(FunctionStmt<'a>),
    Variable(VariableStmt<'a>),

    // Control flow
    If(IfStmt<'a>),
    Switch(SwitchStmt<'a>),

    // Loops
    While(WhileStmt<'a>),
    DoWhile(DoWhileStmt<'a>),
    For(ForStmt),

    Typedef(TypedefStmt<'a>),

    Return(ReturnStmt<'a>),
    Break(BreakStmt<'a>),
    Continue(ContinueStmt<'a>),
    Goto(GotoStmt<'a>),

    Block(BlockStmt<'a>),
    
    Expression(Expression<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'a> {
    LiteralString(&'a str),
    LiteralChar(char),

    LiteralShort(i16),
    LiteralInt(i32),
    LiteralLong(i64),
    LiteralFloat(f32),
    LiteralDouble(f64),

    Ident(Ident<'a>),

    Deref(&'a Expression<'a>),
    AddrOf(&'a Expression<'a>),

    Sizeof(&'a Expression<'a>),
    
    Call(CallExpr<'a>),
    BinaryOperation(BinOpExpr<'a>),
    UnaryOperation(UnOpExpr<'a>),

    // Assignment
    CompoundAssignment(CompoundAssignmentExpr<'a>),
    Assignment(AssignmentExpr<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructStmt<'a> {
    pub name: Option<Ident<'a>>,
    pub fields: Vec<Field<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field<'a> {
    pub name: Ident<'a>,
    pub _type: Type<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumStmt<'a> {
    pub name: Option<Ident<'a>>,
    pub variants: Vec<Ident<'static>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnionStmt<'a> {
    pub name: Option<Ident<'a>>,
    pub fields: Vec<Field<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionStmt<'a> {
    pub name: Ident<'a>,
    pub is_volatile: bool,
    pub is_static: bool,
    pub is_inline: bool,
    pub args: Vec<Field<'a>>,
    pub ret_type: Type<'a>,
    pub body: Option<BlockStmt<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableStmt<'a> {
    pub name: Ident<'a>,
    pub is_volatile: bool,
    pub is_const: bool,
    pub is_static: bool,
    pub is_register: bool,
    pub _type: Type<'a>,
    pub val: Option<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IfType {
    If,
    ElseIf,
    Else,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt<'a> {
    pub if_type: IfType,
    pub cond: Option<Expression<'a>>,
    pub block: BlockStmt<'a>,
    pub alt: Option<&'a IfStmt<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStmt<'a> {
    pub comp_val: Expression<'a>,
    pub cases: Vec<CaseStmt<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseStmt<'a> {
    pub comp_val: Expression<'a>,
    pub block: BlockStmt<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStmt<'a> {
    pub cond: Expression<'a>,
    pub block: BlockStmt<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DoWhileStmt<'a> {
    pub cond: Expression<'a>,
    pub block: BlockStmt<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStmt {
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypedefStmt<'a> {
    pub name: Ident<'a>,
    pub _type: &'a Statement<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt<'a> {
    pub val: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStmt<'a> {
    pub label: Option<Ident<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContinueStmt<'a> {
    pub label: Option<Ident<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GotoStmt<'a> {
    pub label: Option<Ident<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStmt<'a> {
    pub block: Vec<Statement<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr<'a> {
    pub name: Ident<'a>,
    pub args: Vec<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinOpExpr<'a> {
    pub left: Box<Expression<'a>>,
    pub right: Box<Expression<'a>>,
    pub operator: BinOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnOpExpr<'a> {
    pub left: &'a Expression<'a>,
    pub operator: UnOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompoundAssignmentExpr<'a> {
    pub ident: Ident<'a>,
    pub val: &'a Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpr<'a> {
    pub ident: Ident<'a>,
    pub val: &'a Expression<'a>,
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