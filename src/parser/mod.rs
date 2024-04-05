#![macro_use]

use crate::lexer::{tokens::Token, Lexer};

use self::ast::{BlockStmt, Expression, FunctionStmt, Statement, VariableStmt};

use core::option::Option;

pub mod ast;

#[macro_export]
macro_rules! expect_peek {
    ($tok:expr,$pat:pat,$fail:expr) => {{
        if !matches!($tok, $pat) {
            $fail($tok);
            false
        } else {
            true
        }
    }};
}

#[macro_export]
macro_rules! parser_error {
    ($($arg:tt)+) => {{
        use colored::Colorize;

        eprintln!("{}: {}", "Parser Error".red(), format_args!($($arg)+))
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
                tok => todo!("{:?}", tok),
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
                Some(Token::Assign) | Some(Token::Semicolon) => {
                    return Statement::Variable(self.parse_variable())
                }
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
        self.next_tok();
        self.next_tok();
        self.next_tok();
        dbg!(self.cur_tok());
        let block = self.parse_block();
        FunctionStmt {
            name: ident.to_string(),
            is_volatile,
            is_static,
            is_inline,
            args: Vec::new(),
            ret_type: _type.into(),
            body: Some(block),
        }
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
        if !expect_peek!(self.peek_tok().copied(), Some(Token::Assign), |_| ()) {
            if expect_peek!(self.peek_tok(), Some(Token::Semicolon), |_| {
                panic!("EEEEE")
            }) {
                let var = VariableStmt {
                    name: ident.into(),
                    _type: _type.to_string(),
                    val: None,
                    is_const,
                    is_static,
                    is_register,
                    is_volatile,
                };
                dbg!(&var);
                return var;
            }
        };
        // Skip Token::Assign
        self.next_tok();
        self.next_tok();

        let val = Expression::LiteralString("eee".into());

        expect_peek!(self.peek_tok(), Some(Token::Semicolon), |_| ());
        self.next_tok();
        self.next_tok();

        VariableStmt {
            name: ident.into(),
            val: Some(val),
            _type: _type.into(),
            is_const,
            is_static,
            is_volatile,
            is_register,
        }
    }

    /// First tok needs to be Token::LCurly
    fn parse_block(&mut self) -> BlockStmt {
        let mut block = Vec::new();
        self.next_tok();
        while self.peek_tok() != Some(&Token::RCurly) {
            block.push(self.parse_stmt());
        }
        BlockStmt { block }
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
