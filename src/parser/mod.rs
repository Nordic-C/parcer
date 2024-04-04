use crate::lexer::{tokens::Token, Lexer};

use self::ast::Statement;

pub mod ast;

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
                Token::Auto => todo!(),
                Token::Const => todo!(),
                Token::Static => todo!(),
                Token::Register => todo!(),
                Token::Volatile => todo!(),
                Token::Restrict => todo!(),
                Token::Inline => todo!(),
                Token::Signed => todo!(),
                Token::Unsigned => todo!(),
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
                _ => todo!()
            },
            None => todo!(),
        }
    }

    fn cur_tok(&self) -> Option<&Token<'a>> {
        self.lexer.tokens.get(self.tok_index)
    }

    fn peek_tok(&self) -> Option<&Token<'a>> {
        self.lexer.tokens.get(self.tok_index + 1)
    }

    fn next(&mut self) {
        self.tok_index += 1;
    }
}
