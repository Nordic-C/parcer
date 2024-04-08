use bumpalo::Bump;

use crate::{
    expect_peek,
    lexer::{tokens::Token, Lexer},
    parser::ast::Type,
    valid_var_or_func,
};

use self::ast::{
    BinOpExpr, BinOperator, BlockStmt, Expression, FunctionStmt, Ident, PreOperator, PrefixExpr,
    Statement, VariableStmt,
};

use core::option::Option;
use std::collections::HashSet;

pub mod ast;
mod util;

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

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub variables: HashSet<Ident<'a>>,
    pub types: HashSet<Ident<'a>>,
    arena: &'a Bump,
    tok_index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, arena: &'a Bump) -> Self {
        Self {
            lexer,
            tok_index: 0,
            variables: HashSet::new(),
            types: HashSet::new(),
            arena,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement<'a>> {
        todo!()
    }

    pub fn parse_stmt(&mut self) -> Statement<'a> {
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
                Token::Ident(ident) => {dbg!(ident); self.determine_ident()},
                tok => todo!("{:?}", tok),
            },
            None => todo!(),
        }
    }

    fn determine_ident(&mut self) -> Statement<'a> {
        let mut index = self.tok_index;
        loop {
            match self.lexer.tokens.get(index) {
                Some(Token::LParent) => return Statement::Function(self.parse_function()),
                Some(Token::Assign) | Some(Token::Semicolon) => {
                    return Statement::Variable(self.parse_variable())
                }
                Some(Token::Ident(ident)) => {
                    let is_expr = self.variables.contains(ident);
                    match self.lexer.tokens.get(index + 1) {
                        // Pointer or multiplication
                        Some(Token::Asterisk) => match self.lexer.tokens.get(index + 2) {
                            Some(Token::Ident(id)) => {
                                if is_expr {
                                    return Statement::Expression(
                                        self.parse_expression(Precedence::Lowest),
                                    );
                                } else {
                                    println!("ID: {}", id);
                                    return self.parse_var_or_func();
                                }
                            }
                            valid_var_or_func!() => return self.parse_var_or_func(),
                            _ => {
                                return Statement::Expression(
                                    self.parse_expression(Precedence::Lowest),
                                )
                            }
                        },
                        Some(Token::Ident(id)) => {
                            dbg!(id);
                            dbg!(self.peek_tok());
                            return self.parse_var_or_func();
                        }
                        valid_var_or_func!() => return self.parse_var_or_func(),
                        _ => {
                            return Statement::Expression(self.parse_expression(Precedence::Lowest))
                        }
                    }
                }
                None => panic!("Eof"),
                _ => (),
            }
            index += 1;
        }
    }

    fn parse_expression(&mut self, prec: Precedence) -> Expression<'a> {
        let first_expr = self.parse_expr();

        let mut left_expr = first_expr;

        // peek tok as prec
        while !self.peek_is_end() && prec < Self::tok_to_prec(self.peek_tok().unwrap()) {
            self.next_tok();
            // Unwrap here might not be safe. Observe this
            left_expr = self.parse_infix(left_expr);
        }

        left_expr
    }

    fn parse_infix(&mut self, left: Expression<'a>) -> Expression<'a> {
        match self.cur_tok() {
            Some(tok) => match tok {
                Token::Equals | Token::Plus | Token::Minus | Token::Asterisk | Token::Divide => {
                    self.parse_infix_expr(left)
                }
                //Token::LParent => Expression::Call(self.parse_call_expr(left)),
                // Token::LSquare => self.parse_index_expr(left),
                _ => panic!("Invalid for parsing an infix expr: {:#?}", left),
            },
            _ => todo!(),
        }
    }

    fn parse_expr(&mut self) -> Expression<'a> {
        match self.cur_tok() {
            Some(tok) => match tok {
                Token::LitString(str) => {
                    let string = str.trim_matches('"');
                    Expression::LiteralString(string)
                }
                Token::LitInt(int) => Expression::LiteralInt(int.parse().unwrap()),
                Token::LitFloat(float) => Expression::LiteralFloat(float.parse().unwrap()),
                Token::LitChar(char) => Expression::LiteralChar(char.chars().nth(1).unwrap()),
                Token::Ident(ident) => Expression::Ident(ident),
                Token::ExclamMark
                | Token::Increment
                | Token::Decrement
                | Token::Ampersand
                | Token::Asterisk
                | Token::Plus
                | Token::Minus
                | Token::Sizeof
                | Token::LParent => self.parse_prefix_expr(),
                Token::LCurly => todo!(),
                tok => todo!("{:?}", tok),
            },
            None => todo!(),
        }
    }

    fn parse_prefix_expr(&mut self) -> Expression<'a> {
        let op = self.tok_to_pre_op();
        self.next_tok();
        let val = self.parse_expression(Precedence::Prefix);
        Expression::Prefix(PrefixExpr {
            op,
            val: self.arena.alloc(val),
        })
    }

    fn parse_infix_expr(&mut self, left: Expression<'a>) -> Expression<'a> {
        let op = Self::tok_to_bin_op(self.cur_tok().unwrap());
        let prec = Self::tok_to_prec(self.cur_tok().unwrap());
        self.next_tok();
        let right: Expression<'a> = self.parse_expression(prec);
        Expression::BinaryOperation(BinOpExpr {
            left: self.arena.alloc(left),
            right: self.arena.alloc(right),
            operator: op,
        })
    }

    fn parse_var_or_func(&mut self) -> Statement<'a> {
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

    fn parse_function(&mut self) -> FunctionStmt<'a> {
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
        self.next_tok();
        self.next_tok();
        self.next_tok();
        let block = self.parse_block();
        let func = FunctionStmt {
            name: ident,
            is_volatile,
            is_static,
            is_inline,
            args: Vec::new(),
            ret_type: Type::Ident(_type.into()),
            body: Some(block),
        };
        dbg!(&func);
        self.reset_variables();
        func
    }

    fn parse_variable(&mut self) -> VariableStmt<'a> {
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
                            _type = Some(Type::Ident(*ident));
                        } else {
                            break (_type.unwrap(), *ident);
                        }
                    }
                    Some(Token::Asterisk) => {
                        _type = Some(Type::Pointer(self.arena.alloc(_type.unwrap())));
                    }
                    Some(Token::Auto) => _type = Some(Type::Ident("auto")),
                    Some(Token::Const) => is_const = true,
                    Some(Token::Static) => is_static = true,
                    Some(Token::Volatile) => is_volatile = true,
                    Some(Token::Register) => is_register = true,
                    Some(_) => panic!(),
                    None => panic!("EOF"),
                }
                self.next_tok()
            }
        };
        if !expect_peek!(self.peek_tok().copied(), Some(Token::Assign), |_| ()) {
            if expect_peek!(self.peek_tok(), Some(Token::Semicolon), |_| {
                panic!("EEEEE")
            }) {
                self.next_tok();
                self.next_tok();
                self.variables.insert(ident);
                let var = VariableStmt {
                    name: ident,
                    _type: _type,
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

        let val = self.parse_expression(Precedence::Lowest);

        expect_peek!(self.peek_tok(), Some(Token::Semicolon), |_| ());
        self.next_tok();
        self.next_tok();

        self.variables.insert(ident);

        let var = VariableStmt {
            name: ident,
            val: Some(val),
            _type: _type,
            is_const,
            is_static,
            is_volatile,
            is_register,
        };
        var
    }

    /// First tok needs to be Token::LCurly
    fn parse_block(&mut self) -> BlockStmt<'a> {
        let mut block = Vec::new();
        self.next_tok();
        while self.peek_tok() != Some(&Token::RCurly) {
            block.push(self.parse_stmt());
        }
        BlockStmt { block }
    }

    fn tok_to_bin_op(tok: &Token<'a>) -> BinOperator {
        match tok {
            Token::Plus => BinOperator::Add,
            Token::Minus => BinOperator::Sub,
            Token::Asterisk => BinOperator::Mul,
            Token::Divide => BinOperator::Div,
            Token::Mod => BinOperator::Mod,
            Token::LeftShift => BinOperator::LSh,
            Token::RightShift => BinOperator::RSh,
            Token::Ampersand => BinOperator::BAnd,
            Token::BOr => BinOperator::BOr,
            Token::XOr => BinOperator::BXor,
            _ => todo!(),
        }
    }

    fn tok_to_pre_op(&mut self) -> PreOperator<'a> {
        match self.cur_tok() {
            Some(tok) => match tok {
                Token::Plus => PreOperator::Pos,
                Token::Minus => PreOperator::Neg,
                Token::ExclamMark => PreOperator::Not,
                Token::Not => PreOperator::BNot,
                Token::Asterisk => PreOperator::Deref,
                Token::Sizeof => PreOperator::SizeOf,
                Token::Ampersand => PreOperator::AddrOf,
                Token::LParent => {
                    self.next_tok();
                    let _type = Type::Ident(match self.cur_tok() {
                        Some(Token::Ident(ident)) => ident,
                        _ => todo!("Implement support for more complicated types when casting"),
                    });
                    self.next_tok();
                    PreOperator::Cast(_type)
                }
                _ => todo!(),
            },
            None => todo!(),
        }
    }

    fn tok_to_prec(tok: &Token<'a>) -> Precedence {
        match tok {
            Token::Comma => Precedence::Comma,
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
            Token::QuestionMark | Token::Colon => Precedence::Ternary,
            Token::Or => Precedence::Or,
            Token::And => Precedence::And,
            Token::BOr => Precedence::BOr,
            Token::XOr => Precedence::BXor,
            Token::Ampersand => Precedence::BAnd,
            Token::Equals | Token::NEquals => Precedence::Equals,
            Token::GreaterThan | Token::LessThan | Token::GTEquals | Token::LTEquals => {
                Precedence::Relational
            }
            Token::LeftShift | Token::RightShift => Precedence::Shift,
            Token::Plus | Token::Minus => Precedence::Add,
            Token::Asterisk | Token::Divide | Token::Mod => Precedence::Mul,
            Token::Increment
            | Token::Decrement
            | Token::LParent
            | Token::LSquare
            | Token::Dot
            | Token::Arrow => Precedence::Postfix,
            tok => todo!("precedence for: {tok:?}"),
        }
    }

    #[inline(always)]
    fn cur_tok(&self) -> Option<&Token<'a>> {
        self.lexer.tokens.get(self.tok_index)
    }

    #[inline(always)]
    fn peek_tok(&self) -> Option<&Token<'a>> {
        self.lexer.tokens.get(self.tok_index + 1)
    }

    #[inline(always)]
    pub fn next_tok(&mut self) {
        self.tok_index += 1;
    }

    #[inline(always)]
    fn peek_is_end(&self) -> bool {
        matches!(self.peek_tok(), Some(Token::Semicolon) | None)
    }

    #[inline(always)]
    fn reset_variables(&mut self) {
        self.variables.clear();
    }
}
