pub type Ident = String;
pub type Type = String;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    // Data types
    Struct(StructStmt),
    Enum(EnumStmt),
    Union(UnionStmt),

    Function(FunctionStmt),
    Variable(VariableStmt),

    // Control flow
    If(IfStmt),
    Switch(SwitchStmt),

    // Loops
    While(WhileStmt),
    DoWhile(DoWhileStmt),
    For(ForStmt),

    Typedef(TypedefStmt),

    Return(ReturnStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Goto(GotoStmt),

    Block(BlockStmt),
    
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    LiteralString(String),
    LiteralChar(char),

    LiteralShort(i16),
    LiteralInt(i32),
    LiteralLong(i64),
    LiteralFloat(f32),
    LiteralDouble(f64),

    Pointer(Box<Expression>),

    Sizeof(Box<Expression>),
    
    Call(CallExpr),
    BinaryOperation(BinOpExpr),
    UnaryOperation(UnOpExpr),

    // Assignment
    CompoundAssignment(CompoundAssignmentExpr),
    Assignment(AssignmentExpr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructStmt {
    pub name: Option<Ident>,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: Ident,
    pub _type: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumStmt {
    pub name: Option<Ident>,
    pub variants: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnionStmt {
    pub name: Option<Ident>,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionStmt {
    pub name: Ident,
    pub is_volatile: bool,
    pub is_static: bool,
    pub is_inline: bool,
    pub args: Vec<Field>,
    pub ret_type: Type,
    pub body: BlockStmt,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableStmt {
    pub name: Ident,
    pub is_volatile: bool,
    pub is_const: bool,
    pub is_static: bool,
    pub is_register: bool,
    pub _type: Type,
    pub val: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IfType {
    If,
    ElseIf,
    Else,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt {
    pub if_type: IfType,
    pub cond: Option<Expression>,
    pub block: BlockStmt,
    pub alt: Option<Box<IfStmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStmt {
    pub comp_val: Expression,
    pub cases: Vec<CaseStmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseStmt {
    pub comp_val: Expression,
    pub block: BlockStmt,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStmt {
    pub cond: Expression,
    pub block: BlockStmt,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DoWhileStmt {
    pub cond: Expression,
    pub block: BlockStmt,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStmt {
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypedefStmt {
    pub name: Ident,
    pub _type: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt {
    pub val: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStmt {
    pub label: Option<Ident>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContinueStmt {
    pub label: Option<Ident>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GotoStmt {
    pub label: Option<Ident>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStmt {
    pub block: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub name: Ident,
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinOpExpr {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: BinOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnOpExpr {
    pub left: Box<Expression>,
    pub operator: UnOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompoundAssignmentExpr {
    pub ident: Ident,
    pub val: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpr {
    pub ident: Ident,
    pub val: Box<Expression>,
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