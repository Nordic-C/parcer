use crate::lexer::tokens::Token;

use super::{Expression, Parser};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Precedence {
    Lowest,
    Comma,
    Assign,
    Ternary,
    Or,
    And,
    BOr,
    BXor,
    BAnd,
    Equals,
    Relational,
    Shift,
    Add,
    Mul,
    Prefix,
    Postfix,
}

impl<'a, 's: 'a> Parser<'a, 's> {
    pub(super) fn parse_expr(&mut self) -> Option<Expression<'a>> {
        match self.cur_tok()? {
            Token::LitString(str) => Some(Expression::LiteralString(*str)),
            Token::LitInt(int) => Some(Expression::LiteralInt(int.parse().unwrap())),
            Token::LitFloat(float) => Some(Expression::LiteralFloat(float.parse().unwrap())),
            Token::LitChar(char) => Some(Expression::LiteralChar(char.parse().unwrap())),
            Token::Ident(ident) => Some(Expression::Ident(*ident)),
            _ => todo!(),
        }
    }
}
