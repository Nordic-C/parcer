#![macro_use]

use crate::{
    expect_peek,
    lexer::{tokens::Token, Lexer},
    parser::ast::Type,
};

use self::ast::{
    BinOpExpr, BinOperator, BlockStmt, Expression, FunctionStmt, Statement, VariableStmt,
};

use core::option::Option;

pub mod ast;
mod util;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Precedence {
    Lowest,
    Comma,
    Assign,
    Conditional,
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
    UnaryOp,
    Prefix,
    Postfix,
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
                Token::Ident(_) => match self.peek_tok() {
                    Some(Token::Ident(_)) => self.parse_var_or_func(),
                    _ => Statement::Expression(self.parse_expression(Precedence::Lowest)),
                },
                tok => todo!("{:?}", tok),
            },
            None => todo!(),
        }
    }

    fn parse_expression(&mut self, prec: Precedence) -> Expression<'a> {
        let prefix = self.parse_prefix();

        let mut left_expr = prefix;

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

    fn parse_prefix(&mut self) -> Expression<'a> {
        match self.cur_tok() {
            Some(tok) => match tok {
                Token::Sizeof => todo!(),
                Token::LitString(str) => {
                    let string = str.trim_matches('"');
                    Expression::LiteralString(string)
                }
                Token::LitInt(int) => Expression::LiteralInt(int.parse().unwrap()),
                Token::LitFloat(float) => Expression::LiteralFloat(float.parse().unwrap()),
                Token::LitChar(char) => Expression::LiteralChar(char.chars().nth(1).unwrap()),
                Token::Ident(ident) => Expression::Ident(ident),
                Token::ExclamMark => todo!(),
                Token::Increment => todo!(),
                Token::Decrement => todo!(),
                Token::Ampersand => todo!(),
                Token::Asterisk => todo!(),
                Token::LCurly => todo!(),
                Token::LParent => todo!(),
                tok => todo!("{:?}", tok),
            },
            None => todo!(),
        }
    }

    fn parse_infix_expr(&mut self, left: Expression<'a>) -> Expression<'a> {
        let op = Self::tok_to_bin_op(self.cur_tok().unwrap());
        let prec = Self::tok_to_prec(self.cur_tok().unwrap());
        self.next_tok();
        let right: Expression<'a> = self.parse_expression(prec);
        Expression::BinaryOperation(BinOpExpr {
            left: Box::new(left),
            right: Box::new(right),
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
        dbg!(is_volatile, is_static, is_inline, ident, _type);
        self.next_tok();
        self.next_tok();
        self.next_tok();
        dbg!(self.cur_tok());
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
                    _type: Type::Ident(_type),
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

        let var = VariableStmt {
            name: ident.into(),
            val: Some(val),
            _type: Type::Ident(_type.into()),
            is_const,
            is_static,
            is_volatile,
            is_register,
        };
        dbg!(&var);
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
            _ => todo!()
        }
    }

    fn tok_to_prec(tok: &Token<'a>) -> Precedence {
        match tok {
            Token::Comma => Precedence::Comma,
            Token::Assign => Precedence::Assign,
            Token::And => Precedence::And,
            Token::Or => Precedence::Or,
            Token::Plus | Token::Minus => Precedence::Add,
            Token::Asterisk | Token::Divide => Precedence::Mul,
            tok => todo!("precedence for: {tok:?}")
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
    fn next_tok(&mut self) {
        self.tok_index += 1;
    }

    #[inline(always)]
    fn peek_is_end(&self) -> bool {
        matches!(self.peek_tok(), Some(Token::Semicolon) | None)
    }
}
