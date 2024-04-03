#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    // Preprocessor directives
    Define,
    Undef,
    IfDef,
    IfnDef,
    ElseIf,
    /// Else preprocessor directive
    ElseDir,
    EndIf,

    Include,
    Pragma,
    Error,
    Line,
    Warning,

    // Keywords

    // Types
    Auto,

    // Ints
    Short,
    Int,
    Long,

    // Floats
    Float,
    Double,

    // Other types
    Char,
    Void,
    
    // Variable/function properties
    Const,
    Static,
    Register,
    Volatile,
    Restrict,

    // Number specifier
    Signed,
    Unsigned,

    // Control flow interrupter
    Break,
    Continue,
    Goto,
    Return,
    
    // Data structures
    Enum,
    Struct,
    Union,

    // Regular control flow
    If,
    Else,

    // Loop related
    Do,
    For,
    While,

    // Switch control flow
    Switch,
    Case,
    Default,

    // Other
    Extern,
    Sizeof,
    Typedef,

    // Literals
    LitString(String),
    LitInt(i64),
    LitFloat(f64),
    LitChar(char),
    Ident(String),

    // Symbols

    // Assignment
    Assign,
    AssignPlus,
    AssignMinus,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignBAnd,
    AssignBOr,
    AssignXor,
    AssignLSh,
    AssignRSh,

    // Comparison
    Equals,
    NEquals,
    GTEquals,
    LTEquals,
    LessThan,
    GreaterThan,

    // Logical operations
    ExclamMark,
    And,
    Or,

    // Bitwise operations
    // BAnd, -> see [Ampersand](Token::Ampersand)
    /// Symbol: |
    BOr,
    /// Symbol: ^
    XOr,
    /// Symbol: ~
    Not,
    /// Symbol: <<
    LeftShift,
    /// Symbol: >>
    RightShift,

    // Arithmetic operations
    Plus,
    Minus,
    // Multiply, -> see [Asterisk](Token::Asterisk)
    Divide,
    Mod,

    // Increment and decrement
    Increment,
    Decrement,

    // Misc
    Comma,
    Semicolon,
    Dot,
    Arrow,
    BackSlash,
    Ampersand,
    Asterisk,

    // Brackets
    LSquare,
    RSquare,
    LParent,
    RParent,
    LCurly,
    RCurly,

    Eol,
    Eof,
}