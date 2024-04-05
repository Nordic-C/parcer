#![macro_use]

use crate::lexer::{tokens::Token, Lexer};

use self::ast::{Expression, FunctionStmt, Statement, VariableStmt};

pub mod ast;

#[macro_export]
macro_rules! expect_peek {
    ($tok:expr,$pat:pat) => {{
        if !matches!($tok, $pat) {
            panic!("Expected: {:?} to equal {:?}", $tok, stringify!($pat))
        }
    }};
}

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    tok_index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            tok_index: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        todo!()
    }

    pub fn parse_stmt(&mut self) -> Statement {
        match self.cur_tok() {
            Some(tok) => match tok {
                Token::Auto | Token::Const | Token::Signed | Token::Unsigned | Token::Register => {
                    Statement::Variable(self.parse_variable())
                }
                Token::Inline => Statement::Function(self.parse_function()),
                Token::Static | Token::Volatile => self.parse_var_or_func(),
                Token::Break => todo!(),
                Token::Continue => todo!(),
                Token::Goto => todo!(),
                Token::Return => todo!(),
                Token::Enum => todo!(),
                Token::Struct => todo!(),
                Token::Union => todo!(),
                Token::If => todo!(),
                Token::Else => todo!(),
                Token::Do => todo!(),
                Token::For => todo!(),
                Token::While => todo!(),
                Token::Switch => todo!(),
                Token::Extern => todo!(),
                Token::Typedef => todo!(),
                _ => Statement::Expression(self.parse_expression()),
            },
            None => todo!(),
        }
    }

    fn parse_expression(&mut self) -> Expression {
        todo!()
    }

    fn parse_var_or_func(&mut self) -> Statement {
        let mut index = self.tok_index;
        loop {
            match self.lexer.tokens.get(index) {
                Some(Token::LParent) => return Statement::Function(self.parse_function()),
                Some(Token::Assign) => return Statement::Variable(self.parse_variable()),
                None => panic!("Eof"),
                _ => (),
            }
            index += 1;
        }
    }

    fn parse_function(&mut self) -> FunctionStmt {
        let mut is_volatile = false;
        let mut is_static = false;
        let mut is_inline = false;
        let (_type, ident) = {
            // Set type to none until we find it
            // After we find it, set is to Some(...)
            // and only search for variable name then
            let mut _type = None;
            loop {
                match self.cur_tok() {
                    Some(Token::Ident(ident)) => {
                        if _type.is_none() {
                            _type = Some(*ident);
                        } else {
                            break (_type.unwrap(), *ident);
                        }
                    }
                    Some(Token::Auto) => _type = Some("auto"),
                    Some(Token::Inline) => is_inline = true,
                    Some(Token::Static) => is_static = true,
                    Some(Token::Volatile) => is_volatile = true,
                    None => panic!("EOF"),
                    _ => (),
                }
                self.next_tok()
            }
        };
        dbg!(is_volatile, is_static, is_inline, ident, _type);
        todo!()
    }

    fn parse_variable(&mut self) -> VariableStmt {
        let mut is_const = false;
        let mut is_volatile = false;
        let mut is_static = false;
        let mut is_register = false;
        let (_type, ident) = {
            // Set type to none until we find it
            // After we find it, set is to Some(...)
            // and only search for function name then
            let mut _type = None;
            loop {
                match self.cur_tok() {
                    Some(Token::Ident(ident)) => {
                        if _type.is_none() {
                            _type = Some(*ident);
                        } else {
                            break (_type.unwrap(), *ident);
                        }
                    }
                    Some(Token::Auto) => _type = Some("auto"),
                    Some(Token::Const) => is_const = true,
                    Some(Token::Static) => is_static = true,
                    Some(Token::Volatile) => is_volatile = true,
                    Some(Token::Register) => is_register = true,
                    None => panic!("EOF"),
                    _ => (),
                }
                self.next_tok()
            }
        };
        expect_peek!(self.peek_tok(), Some(Token::Assign));
        // Skip Token::Assign
        self.next_tok();
        self.next_tok();

        let val = self.parse_expression();

        VariableStmt {
            name: ident.into(),
            val,
            _type: _type.into(),
            is_const,
            is_static,
            is_volatile,
            is_register,
        }
    }

    fn cur_tok(&self) -> Option<&Token<'a>> {
        self.lexer.tokens.get(self.tok_index)
    }

    fn peek_tok(&self) -> Option<&Token<'a>> {
        self.lexer.tokens.get(self.tok_index + 1)
    }

    fn next_tok(&mut self) {
        self.tok_index += 1;
    }
}
