mod tests;
pub mod tokens;

use self::tokens::Token;

pub struct Lexer {
    src: String,
    cur_char: Option<char>,
    cur_pos: usize,
    next_pos: usize,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        let mut lexer = Self {
            src,
            cur_char: None,
            cur_pos: 0,
            next_pos: 0,
        };
        lexer.next_char();
        lexer
    }

    pub fn tokenize(&mut self) -> Token {
        self.skip_whitespace();
        match self.cur_char {
            Some(char) => match char {
                c if c.is_alphabetic() || c == '_' => self.tokenize_ident(),
                c if c.is_numeric() => self.tokenize_number(),
                c if c == '.' => self.tokenize_float(),
                c if c == '"' => self.tokenize_string(),
                _ => self.tokenize_symbols(),
            },
            None => Token::Eof,
        }
    }

    fn tokenize_float(&mut self) -> Token {
        let first_pos = self.cur_pos;
        self.next_char();
        while let Some(ch) = self.cur_char {
            if ch.is_numeric() || ch == '_' {
                self.next_char();
            } else {
                break;
            }
        }
        let number: String = self.src[first_pos..self.cur_pos].into();
        Token::LitFloat(number.parse().unwrap())
    }

    fn tokenize_number(&mut self) -> Token {
        let first_pos = self.cur_pos;
        let mut found_fp = false;
        while let Some(ch) = self.cur_char {
            if ch.is_numeric() || ch == '_' {
                self.next_char();
            } else if ch == '.' {
                if !found_fp {
                    found_fp = true;
                    self.next_char();
                } else {
                    panic!("Found second decimal point in number");
                }
            } else {
                break;
            }
        }
        let number: String = self.src[first_pos..self.cur_pos].into();
        if found_fp {
            Token::LitFloat(number.parse().unwrap())
        } else {
            Token::LitInt(number.parse().unwrap())
        }
    }

    fn tokenize_string(&mut self) -> Token {
        self.next_char();
        let first_pos = self.cur_pos;
        while let Some(ch) = self.cur_char {
            if ch != '"' {
                self.next_char();
            } else {
                break;
            }
        }
        let string: String = self.src[first_pos..self.cur_pos].into();
        self.next_char();
        Token::LitString(string)
    }

    fn tokenize_ident(&mut self) -> Token {
        let first_pos = self.cur_pos;
        while let Some(ch) = self.cur_char {
            if ch.is_alphanumeric() || ch == '_' {
                self.next_char();
            } else {
                break;
            }
        }
        let ident: String = self.src[first_pos..self.cur_pos].into();
        match ident.as_str() {
            "auto" => Token::Auto,
            "short" => Token::Short,
            "int" => Token::Int,
            "long" => Token::Long,
            "float" => Token::Float,
            "double" => Token::Double,
            "char" => Token::Char,
            "void" => Token::Void,
            "const" => Token::Const,
            "static" => Token::Static,
            "register" => Token::Register,
            "volatile" => Token::Volatile,
            "restrict" => Token::Restrict,
            "signed" => Token::Signed,
            "unsigned" => Token::Unsigned,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "goto" => Token::Goto,
            "return" => Token::Return,
            "enum" => Token::Enum,
            "struct" => Token::Struct,
            "union" => Token::Union,
            "if" => Token::If,
            "else" => Token::Else,
            "do" => Token::Do,
            "for" => Token::For,
            "while" => Token::While,
            "switch" => Token::Switch,
            "case" => Token::Case,
            "default" => Token::Default,
            "extern" => Token::Extern,
            "sizeof" => Token::Sizeof,
            "typedef" => Token::Typedef,
            _ => Token::Ident(ident),
        }
    }

    fn tokenize_symbols(&mut self) -> Token {
        let tok = match self.cur_char {
            Some(ch) => match ch {
                '=' => {
                    let ch = self.next_symbol_char();
                    match ch {
                        '=' => Token::Equals,
                        _ => Token::Assign,
                    }
                }
                '+' => {
                    let ch = self.next_symbol_char();
                    match ch {
                        '+' => Token::Increment,
                        '=' => Token::AssignPlus,
                        _ => Token::Plus,
                    }
                }
                '-' => {
                    let ch = self.next_symbol_char();
                    match ch {
                        '-' => Token::Decrement,
                        '=' => Token::AssignMinus,
                        '>' => Token::Arrow,
                        _ => Token::Minus,
                    }
                }
                '*' => {
                    let ch = self.next_symbol_char();
                    match ch {
                        '=' => Token::AssignMul,
                        _ => Token::Asterisk,
                    }
                }
                '/' => {
                    let ch = self.next_symbol_char();
                    match ch {
                        '=' => Token::AssignDiv,
                        _ => Token::Divide,
                    }
                }
                '%' => Token::Mod,
                '(' => Token::LParent,
                ')' => Token::RParent,
                '{' => Token::LCurly,
                '}' => Token::RCurly,
                '[' => Token::LSquare,
                ']' => Token::RSquare,
                '&' => {
                    let ch = self.next_symbol_char();
                    match ch {
                        '&' => Token::And,
                        '=' => Token::AssignBAnd,
                        _ => Token::Ampersand,
                    }
                }
                '|' => {
                    let ch = self.next_symbol_char();
                    match ch {
                        '|' => Token::Or,
                        '=' => Token::AssignBOr,
                        _ => Token::BOr,
                    }
                }
                '^' => {
                    let ch = self.next_symbol_char();
                    match ch {
                        '=' => Token::AssignXor,
                        _ => Token::XOr,
                    }
                }
                '~' => Token::Not,
                ',' => Token::Comma,
                '.' => Token::Dot,
                '!' => {
                    let ch = self.next_symbol_char();
                    match ch {
                        '=' => Token::NEquals,
                        _ => Token::ExclamMark,
                    }
                }
                '<' => {
                    let first_ch = self.next_symbol_char();
                    let second_ch = self.next_symbol_char();
                    match first_ch {
                        '<' => {
                            match second_ch {
                                '=' => Token::AssignLSh,
                                _ => Token::LeftShift,
                            }
                        }
                        '=' => Token::LTEquals,
                        _ => Token::LessThan,
                    }
                }
                '>' => {
                    let first_ch = self.next_symbol_char();
                    let second_ch = self.next_symbol_char();
                    match first_ch {
                        '>' => {
                            match second_ch {
                                '=' => Token::AssignRSh,
                                _ => Token::RightShift,
                            }
                        }
                        '=' => Token::GTEquals,
                        _ => Token::GreaterThan,
                    }
                }
                ';' => Token::Semicolon,
                '\n' => Token::Eol,
                c => todo!("{c:?}"),
            },
            None => panic!("cur char is none"),
        };
        self.next_char();
        tok
    }

    fn next_symbol_char(&mut self) -> char {
        loop {
            if let Some(ch) = self.cur_char {
                if ch != ' ' {
                    return ch;
                }
            } else {
                self.next_char();
            }
        }
    }

    fn skip_whitespace(&mut self) {
        match self.cur_char {
            Some(mut ch) => {
                while ch.is_whitespace() && ch != '\n' {
                    self.next_char();
                    match self.cur_char {
                        Some(cch) => ch = cch,
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    fn next_char(&mut self) {
        self.cur_pos = self.next_pos;
        self.cur_char = self.src.chars().nth(self.cur_pos);
        self.next_pos += 1;
    }
}
