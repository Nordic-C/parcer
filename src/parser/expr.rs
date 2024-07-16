use std::f32::INFINITY;

use crate::{ast::expr::{Expression, InOperator}, lexer::tokens::Token, parser_error};

use super::Parser;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(super) enum Precedence {
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

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(super) enum PrecedencePos {
    Prefix,
    Infix,
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
            Token::Sizeof => todo!(),
            Token::ExclamMark => todo!(),
            Token::BOr => todo!(),
            Token::XOr => todo!(),
            Token::Not => todo!(),
            Token::Plus => todo!(),
            Token::Minus => todo!(),
            Token::Increment => todo!(),
            Token::Decrement => todo!(),
            Token::Ampersand => todo!(),
            Token::Asterisk => todo!(),
            Token::LParent => todo!(),
            tok => {
                parser_error!("Cannot parse expression or statement from: {tok:?}");
                panic!()
            }
        }
    }

    fn tok_to_in_op(tok: &Token) -> Option<InOperator> {
        match tok {
            Token::Assign => Some(InOperator::Assign),
            Token::AssignAdd => Some(InOperator::AssignAdd),
            Token::AssignSub => Some(InOperator::AssignSub),
            Token::AssignMul => Some(InOperator::AssignMul),
            Token::AssignDiv => Some(InOperator::AssignDiv),
            Token::AssignMod => Some(InOperator::AssignMod),
            Token::AssignBAnd => Some(InOperator::AssingBAnd),
            Token::AssignBOr => Some(InOperator::AssignBOr),
            Token::AssignXor => Some(InOperator::AssignBXor),
            Token::AssignLSh => Some(InOperator::AssignLsh),
            Token::AssignRSh => Some(InOperator::AssignRsh),
            Token::Equals => Some(InOperator::Eq),
            Token::NEquals => Some(InOperator::Neq),
            Token::LTEquals => Some(InOperator::LTE),
            Token::GTEquals => Some(InOperator::GTE),
            Token::LessThan => Some(InOperator::LT),
            Token::GreaterThan => Some(InOperator::GT),
            Token::And => Some(InOperator::Add),
            Token::Or => Some(InOperator::Or),
            Token::BOr => Some(InOperator::BOr),
            Token::XOr => Some(InOperator::BXor),
            Token::LeftShift => Some(InOperator::LSh),
            Token::RightShift => Some(InOperator::RSh),
            Token::Plus => Some(InOperator::Add),
            Token::Minus => Some(InOperator::Sub),
            Token::Ampersand => Some(InOperator::Mul),
            Token::Divide => Some(InOperator::Div),
            Token::Mod => Some(InOperator::Mod),
            _ => None
        }
    }

    fn get_precedence(&self, token: &Token<'a>, pos: PrecedencePos) -> Precedence {
        match pos {
            PrecedencePos::Prefix => match token {
                Token::Increment | Token::Decrement | Token::Plus| Token::Minus | Token::ExclamMark | Token::Not | Token::LParent | Token::Asterisk | Token::Ampersand | Token::Sizeof /* | _Alignof */  => Precedence::Prefix,
                Token::Comma => Precedence::Comma,
                _ => Precedence::Lowest,
            },
            PrecedencePos::Infix => match token {
                Token::Asterisk | Token::Divide | Token::Mod => Precedence::Mul,
                Token::Plus | Token::Minus => Precedence::Add,
                Token::GreaterThan | Token::GTEquals | Token::LessThan | Token::LTEquals => {
                    Precedence::Relational
                }
                Token::Equals | Token::NEquals => Precedence::Equals,
                Token::Ampersand => Precedence::BAnd,
                Token::XOr => Precedence::BXor,
                Token::BOr => Precedence::BOr,
                Token::And => Precedence::And,
                Token::Or => Precedence::Or,
                Token::QuestionMark | Token::Colon => Precedence::Ternary,
                Token::Assign
                | Token::AssignAdd
                | Token::AssignSub
                | Token::AssignMul
                | Token::AssignDiv
                | Token::AssignMod
                | Token::AssignLSh
                | Token::AssignRSh
                | Token::AssignBAnd
                | Token::AssignXor
                | Token::AssignBOr => Precedence::Assign,
                Token::Comma => Precedence::Comma,
                _ => Precedence::Lowest,
            },
            PrecedencePos::Postfix => match token {
                Token::Increment
                | Token::Decrement
                | Token::LParent
                | Token::LSquare
                | Token::Dot
                | Token::Arrow => Precedence::Postfix,
                Token::Comma => Precedence::Comma,
                _ => Precedence::Lowest,
            },
        }
    }
}
