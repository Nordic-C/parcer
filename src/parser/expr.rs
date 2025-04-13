use std::collections::HashMap;

use crate::{
    ast::expr::{
        CallExpr, Expression, InOperator, InfixExpr, PostExpr, PostOperator, PreOperator,
        PrefixExpr,
    },
    expect_tok,
    lexer::tokens::Token,
    parser_error,
};

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
    Pre,
    Post,
}

impl<'a, 's: 'a> Parser<'a, 's> {
    pub(super) fn parse_expr(&mut self, prec: Precedence) -> Option<Expression<'a>> {
        let prefix = self.parse_prefix();

        let mut left_expression = prefix;

        while !self.peek_is_end()
            && prec < self.get_precedence(self.peek_tok()?, PrecedencePos::Post)
        {
            self.next_tok();
            left_expression = self.parse_infix(left_expression?);
        }

        left_expression
    }

    fn parse_prefix(&mut self) -> Option<Expression<'a>> {
        match self.cur_tok()? {
            Token::LitString(str) => Some(Expression::LiteralString(*str)),
            Token::LitInt(int) => Some(Expression::LiteralInt(int.parse().unwrap())),
            Token::LitFloat(float) => Some(Expression::LiteralFloat(float.parse().unwrap())),
            Token::LitChar(char) => Some(Expression::LiteralChar(char.parse().unwrap())),
            Token::Ident(ident) => Some(Expression::Ident(*ident)),
            Token::BOr => todo!(),
            Token::XOr => todo!(),
            Token::BNot
            | Token::Sizeof
            | Token::ExclamMark
            | Token::Plus
            | Token::Asterisk
            | Token::Ampersand
            | Token::LParent
            | Token::Increment
            | Token::Decrement
            | Token::Minus => self.parse_prefix_expr(),
            tok => {
                parser_error!("Cannot parse expression or statement from: {tok:?}");
                panic!()
            }
        }
    }

    fn parse_call_expr(&mut self, left: Expression<'a>) -> Option<CallExpr<'a>> {
        let args = self.parse_call_args()?;
        self.next_tok();
        Some(CallExpr {
            val: self.arena.alloc(left),
            args,
        })
    }

    fn parse_call_args(&mut self) -> Option<Vec<Expression<'a>>> {
        let mut args = Vec::new();
        loop {
            self.next_tok();
            let expr = self.parse_expr(Precedence::Lowest)?;
            args.push(expr);
            match self.peek_tok()? {
                Token::Comma => {
                    self.next_tok();
                }
                Token::RParent => return Some(args),
                tok => parser_error!(
                    "Encountered invalid token after function call parameter: {tok:?}"
                ),
            }
        }
    }

    fn parse_infix(&mut self, left_expr: Expression<'a>) -> Option<Expression<'a>> {
        match self.cur_tok()? {
            Token::Equals
            | Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Divide
            | Token::LessThan
            | Token::GreaterThan
            | Token::LTEquals
            | Token::GTEquals => self.parse_infix_expr(left_expr),
            Token::LParent => Some(Expression::Call(self.parse_call_expr(left_expr)?)),
            // Token::LSquare => self.parse_index_expr(left),
            Token::Increment => Some(Expression::Post(PostExpr {
                val: self.arena.alloc(left_expr),
                op: PostOperator::Incr,
            })),
            Token::Decrement => Some(Expression::Post(PostExpr {
                val: self.arena.alloc(left_expr),
                op: PostOperator::Decr,
            })),
            _ => panic!("Invalid for parsing an infix expr: {:#?}", left_expr),
        }
    }

    fn parse_infix_expr(&mut self, left_expr: Expression<'a>) -> Option<Expression<'a>> {
        let op = Self::tok_to_in_op(self.cur_tok()?)?;
        let prec = self.get_precedence(self.cur_tok()?, PrecedencePos::Post);
        self.next_tok();
        let right_expr = self.parse_expr(prec)?;
        Some(Expression::Infix(InfixExpr {
            left: self.arena.alloc(left_expr),
            right: self.arena.alloc(right_expr),
            op,
        }))
    }

    fn parse_prefix_expr(&mut self) -> Option<Expression<'a>> {
        let op = match self.cur_tok()? {
            Token::Plus => PreOperator::Pos,
            Token::Minus => PreOperator::Neg,
            Token::ExclamMark => PreOperator::Not,
            Token::BNot => PreOperator::BNot,
            Token::Asterisk => PreOperator::Deref,
            Token::Sizeof => self.parse_sizeof_expr()?,
            Token::Ampersand => PreOperator::AddrOf,
            // AlignOf
            Token::LParent => self.parse_cast_expr()?,
            Token::Increment => PreOperator::Incr,
            Token::Decrement => PreOperator::Decr,
            other => panic!("Expected operator, got: {other:?} instead"),
        };
        self.next_tok();
        let val = self.arena.alloc(self.parse_expr(Precedence::Prefix)?);
        op.end_expr(self);
        Some(Expression::Prefix(PrefixExpr { op, val }))
    }

    // Cur token is sizeof keyword
    fn parse_sizeof_expr(&mut self) -> Option<PreOperator<'a>> {
        self.next_tok();
        Some(PreOperator::SizeOf)
    }

    /// Cur token is a left parenthesis
    fn parse_cast_expr(&mut self) -> Option<PreOperator<'a>> {
        self.next_tok();
        let _type = self.parse_type()?;
        self.next_tok();
        if expect_tok!(self.peek_tok()?, Token::RParent, |tok| {
            parser_error!(
                "Expected right parenthesis after type for cast, received token: {:#?} instead",
                tok
            )
        }) {
            self.next_tok();
        }
        Some(PreOperator::Cast(_type))
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
            Token::Ampersand => Some(InOperator::BAnd),
            Token::Asterisk => Some(InOperator::Mul),
            Token::Divide => Some(InOperator::Div),
            Token::Mod => Some(InOperator::Mod),
            _ => todo!(),
        }
    }

    fn get_precedence(&self, token: &Token<'a>, pos: PrecedencePos) -> Precedence {
        match pos {
            PrecedencePos::Pre => match token {
                Token::Increment | Token::Decrement | Token::Plus| Token::Minus | Token::ExclamMark | Token::BNot | Token::LParent | Token::Asterisk | Token::Ampersand | Token::Sizeof /* | _Alignof */  => Precedence::Prefix,
                _ => Precedence::Lowest,
            },
            PrecedencePos::Post => match token {
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
                Token::Increment
                | Token::Decrement
                | Token::LParent
                | Token::LSquare
                | Token::Dot
                | Token::Arrow => Precedence::Postfix,
                _ => Precedence::Lowest,
            },
        }
    }
}
