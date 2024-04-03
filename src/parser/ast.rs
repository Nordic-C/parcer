
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
    
    Call(CallExpr),
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
    pub name: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub _type: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumStmt {
    pub name: Option<String>,
    pub variants: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnionStmt {
    pub name: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionStmt {
    pub name: String,
    pub args: Vec<Field>,
    pub ret_type: Type,
    pub body: BlockStmt,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableStmt {
    pub name: String,
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