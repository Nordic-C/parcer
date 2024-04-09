use logos::Logos;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Logos)]
#[logos(skip r#"(?:\/\/[^\n]*|\t|\s|\f|\n)*"#)]
pub enum Token<'a> {
    // Keywords

    // Types
    #[token("auto")]
    Auto,

    // Variable/function properties
    #[token("const")]
    Const,
    #[token("static")]
    Static,
    #[token("register")]
    Register,
    #[token("volatile")]
    Volatile,
    #[token("restrict")]
    Restrict,
    #[token("inline")]
    Inline,

    // Number specifier
    #[token("signed")]
    Signed,
    #[token("unsigned")]
    Unsigned,

    // Control flow interrupter
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("goto")]
    Goto,
    #[token("return")]
    Return,

    // Data structures
    #[token("enum")]
    Enum,
    #[token("struct")]
    Struct,
    #[token("union")]
    Union,

    // Regular control flow
    #[token("if")]
    If,
    #[token("else")]
    Else,

    // Loop related
    #[token("do")]
    Do,
    #[token("for")]
    For,
    #[token("while")]
    While,

    // Switch control flow
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("default")]
    Default,

    // Other
    #[token("extern")]
    Extern,
    #[token("sizeof")]
    Sizeof,
    #[token("typedef")]
    Typedef,

    // Literals
    #[regex(r#""(?:\\.|[^\\"])*""#)]
    LitString(&'a str),
    #[regex(r"-?[0-9]+")]
    LitInt(&'a str),
    #[regex("-?[0-9]+\\.[0-9]+")]
    LitFloat(&'a str),
    #[regex(r#"'[^\\']'"#)]
    LitChar(&'a str),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'a str),

    // Symbols

    // Assignment
    #[token("=")]
    Assign,
    #[token("+=")]
    AssignAdd,
    #[token("-=")]
    AssignSub,
    #[token("*=")]
    AssignMul,
    #[token("/=")]
    AssignDiv,
    #[token("%=")]
    AssignMod,
    #[token("&=")]
    AssignBAnd,
    #[token("|=")]
    AssignBOr,
    #[token("^=")]
    AssignXor,
    #[token(">>=")]
    AssignLSh,
    #[token("<<=")]
    AssignRSh,

    // Comparison
    #[token("==")]
    Equals,
    #[token("!=")]
    NEquals,
    #[token("<=")]
    LTEquals,
    #[token(">=")]
    GTEquals,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,

    // Logical operations
    #[token("!")]
    ExclamMark,
    #[token("&&")]
    And,
    #[token("||")]
    Or,

    // Bitwise operations
    // BAnd, -> see [Ampersand](Token::Ampersand)
    /// Symbol: |
    #[token("|")]
    BOr,
    /// Symbol: ^
    #[token("^")]
    XOr,
    /// Symbol: ~
    #[token("~")]
    Not,
    /// Symbol: <<
    #[token("<<")]
    LeftShift,
    /// Symbol: >>
    #[token(">>")]
    RightShift,

    // Arithmetic operations
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    // Multiply, -> see [Asterisk](Token::Asterisk)
    #[token("/")]
    Divide,
    #[token("%")]
    Mod,

    // Increment and decrement
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,

    // Misc
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(".")]
    Dot,
    #[token("->")]
    Arrow,
    #[token("\\")]
    BackSlash,
    #[token("&")]
    Ampersand,
    #[token("*")]
    Asterisk,
    #[token("?")]
    QuestionMark,
    #[token(":")]
    Colon,

    // Brackets
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token("(")]
    LParent,
    #[token(")")]
    RParent,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
}
