use bumpalo::Bump;

use crate::{
    expect_tok,
    lexer::{tokens::Token, Lexer},
    parser::{
        self,
        ast::{ContinueStmt, LabelStmt, Type},
    },
    parser_error, valid_var_or_func,
};

use self::ast::{
    BinOpExpr, BinOperator, BlockStmt, BreakStmt, Expression, FunctionStmt, Ident, PreOperator,
    PrefixExpr, Statement, VariableStmt,
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

/// Loop interrupters
#[derive(Debug)]
enum LoopInt {
    Break,
    Continue,
}

pub struct Parser<'a, 's> {
    pub lexer: Lexer<'s>,
    pub variables: HashSet<Ident<'a>>,
    pub types: HashSet<Ident<'a>>,
    arena: &'a Bump,
    tok_index: usize,
}

impl<'a, 's: 'a> Parser<'a, 's> {
    pub fn new(lexer: Lexer<'s>, arena: &'a Bump) -> Self {
        Self {
            lexer,
            tok_index: 0,
            variables: HashSet::new(),
            types: HashSet::new(),
            arena,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement<'a>> {
        let mut out = Vec::new();
        while let Some(stmt) = self.parse_stmt() {
            out.push(stmt)
        }
        out
    }

    // NOTE: Statements should always set cur tok to the next token
    pub fn parse_stmt(&mut self) -> Option<Statement<'a>> {
        match self.cur_tok()? {
            Token::Auto | Token::Const | Token::Signed | Token::Unsigned | Token::Register => {
                self.parse_variable().map(|stmt| Statement::Variable(stmt))
            }
            Token::Inline => self.parse_function().map(|stmt| Statement::Function(stmt)),
            Token::Static | Token::Volatile => self.parse_var_or_func(),
            Token::Break => self.parse_loop_interrupter(LoopInt::Break),
            Token::Continue => self.parse_loop_interrupter(LoopInt::Continue),
            Token::Goto => self.parse_goto(),
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
            Token::Ident(ident) => self.determine_ident(ident),
            tok => todo!("{tok:?}"),
        }
    }

    fn determine_ident(&mut self, ident: Ident<'a>) -> Option<Statement<'a>> {
        let is_expr = self.variables.contains(ident);
        match self.lexer.tokens.get(self.tok_index + 1) {
            // Pointer or multiplication
            Some(Token::Asterisk) => match self.lexer.tokens.get(self.tok_index + 2) {
                Some(Token::Ident(id)) => {
                    if is_expr && self.variables.contains(id) {
                        let expr: Option<Expression<'a>> =
                            self.parse_expression(Precedence::Lowest);
                        expr.map(|expr| self.expr_to_stmt(expr))
                    } else {
                        println!("ID: {}", id);
                        self.parse_var_or_func()
                    }
                }
                valid_var_or_func!() => return self.parse_var_or_func(),
                _ => {
                    let expr: Option<Expression<'a>> = self.parse_expression(Precedence::Lowest);
                    expr.map(|expr| self.expr_to_stmt(expr))
                }
            },
            // Variable or function name
            Some(Token::Ident(_)) => {
                return self.parse_var_or_func();
            }
            Some(Token::Colon) => self.parse_label(),
            // Typical function / variable modifiers
            valid_var_or_func!() => self.parse_var_or_func(),
            _ => {
                let expr = self.parse_expression(Precedence::Lowest);
                expr.map(|expr| self.expr_to_stmt(expr))
            }
        }
    }

    fn parse_expression(&mut self, prec: Precedence) -> Option<Expression<'a>> {
        let first_expr = self.parse_raw_expr();

        let mut left_expr = first_expr;

        // peek tok as prec
        while !self.peek_is_end() && prec < Self::tok_to_prec(self.peek_tok().unwrap()) {
            self.next_tok();
            // Unwrap here might not be safe. Observe this
            left_expr = self.parse_infix(left_expr?);
        }

        left_expr
    }

    fn parse_infix(&mut self, left: Expression<'a>) -> Option<Expression<'a>> {
        match self.cur_tok() {
            Some(tok) => match tok {
                Token::Equals | Token::Plus | Token::Minus | Token::Asterisk | Token::Divide => {
                    self.parse_infix_expr(left)
                }
                // Token::LParent => Expression::Call(self.parse_call_expr(left)),
                // Token::LSquare => self.parse_index_expr(left),
                _ => panic!("Invalid for parsing an infix expr: left = {left:#?}, tok = {tok:#?}"),
            },
            _ => todo!(),
        }
    }

    fn parse_label(&mut self) -> Option<Statement<'a>> {
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |err| ());
        self.next_tok();
        let name = *match self.peek_tok()? {
            Token::Ident(ident) => ident,
            _ => unreachable!(),
        };
        expect_tok!(self.peek_tok(), Some(Token::Colon), |err| ());
        self.next_tok();
        Some(Statement::Label(LabelStmt { name }))
    }

    fn parse_raw_expr(&mut self) -> Option<Expression<'a>> {
        match self.cur_tok() {
            Some(tok) => match tok {
                Token::LitString(str) => {
                    let string = str.trim_matches('"');
                    Some(Expression::LiteralString(string))
                }
                Token::LitInt(int) => Some(Expression::LiteralInt(int.parse().unwrap())),
                Token::LitFloat(float) => Some(Expression::LiteralFloat(float.parse().unwrap())),
                Token::LitChar(char) => Some(Expression::LiteralChar(char.chars().nth(1).unwrap())),
                Token::Ident(ident) => Some(Expression::Ident(ident)),
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

    fn parse_prefix_expr(&mut self) -> Option<Expression<'a>> {
        let op = self.tok_to_pre_op();
        self.next_tok();
        let val = self.parse_expression(Precedence::Prefix);
        Some(Expression::Prefix(PrefixExpr {
            op: op?,
            val: self.arena.alloc(val?),
        }))
    }

    fn parse_infix_expr(&mut self, left: Expression<'a>) -> Option<Expression<'a>> {
        let op = Self::tok_to_bin_op(self.cur_tok().unwrap());
        let prec = Self::tok_to_prec(self.cur_tok().unwrap());
        self.next_tok();
        let right: Expression<'a> = self.parse_expression(prec)?;
        Some(Expression::BinaryOperation(BinOpExpr {
            left: self.arena.alloc(left),
            right: self.arena.alloc(right),
            operator: op?,
        }))
    }

    fn parse_var_or_func(&mut self) -> Option<Statement<'a>> {
        let mut index = self.tok_index;
        loop {
            match self.lexer.tokens.get(index) {
                Some(Token::LParent) => return Some(Statement::Function(self.parse_function()?)),
                Some(Token::Assign) | Some(Token::Semicolon) => {
                    return Some(Statement::Variable(self.parse_variable()?))
                }
                Some(_) => index += 1,
                None => panic!("Eof"),
            }
        }
    }

    fn parse_function(&mut self) -> Option<FunctionStmt<'a>> {
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
            body: Some(block?),
        };
        self.next_tok();
        self.reset_variables();
        Some(func)
    }

    fn parse_variable(&mut self) -> Option<VariableStmt<'a>> {
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
        if !expect_tok!(self.peek_tok().copied(), Some(Token::Assign), |_| ()) {
            if expect_tok!(self.peek_tok(), Some(Token::Semicolon), |_| {
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
                return Some(var);
            }
        };
        // Skip Token::Assign
        self.next_tok();
        self.next_tok();

        let val = self.parse_expression(Precedence::Lowest);

        expect_tok!(
            self.peek_tok(),
            Some(Token::Semicolon),
            |tok| parser_error!("Couldnt find semicolon, found {tok:?} instead")
        );
        self.next_tok();
        self.next_tok();

        self.variables.insert(ident);

        Some(VariableStmt {
            name: ident,
            val: Some(val?),
            _type,
            is_const,
            is_static,
            is_volatile,
            is_register,
        })
    }

    fn parse_loop_interrupter(&mut self, int: LoopInt) -> Option<Statement<'a>> {
        let peek_tok = self.peek_tok();
        match &peek_tok {
            Some(Token::Ident(_)) => {
                self.next_tok();
                let label_pos = self.tok_index;
                expect_tok!(self.peek_tok(), Some(Token::Semicolon), |_| ());
                self.next_tok();
                self.next_tok();
                let label = Some(match self.lexer.tokens.get(label_pos) {
                    Some(Token::Ident(label)) => *label,
                    _ => todo!(),
                });
                match int {
                    LoopInt::Break => Some(Statement::Break(BreakStmt{ label })),
                    LoopInt::Continue => Some(Statement::Continue(ContinueStmt { label })),
                }
            },
            Some(Token::Semicolon) => {
                self.next_tok();
                self.next_tok();
                match int {
                    LoopInt::Break => Some(Statement::Break(BreakStmt { label: None })),
                    LoopInt::Continue => Some(Statement::Continue(ContinueStmt { label: None }))
                }
            }
            Some(tok) => panic!("Unexpected token after control flow interrupter. Expected Semicolon or label, received {tok:?} instead"),
            None => panic!(),
        }
    }

    fn parse_goto(&mut self) -> Option<Statement<'a>> {
        expect_tok!(self.peek_tok(), Some(Token::Ident(_)), |tok| parser_error!(
            "Couldnt find name of label, found {tok:?} instead"
        ));
        self.next_tok();
        let label = match *self.cur_tok()? {
            Token::Ident(id) => id,
            tok => todo!("{:?}", tok),
        };
        expect_tok!(
            self.peek_tok(),
            Some(Token::Semicolon),
            |tok| parser_error!("Couldnt find semicolon, found {tok:?} instead")
        );
        self.next_tok();
        self.next_tok();
        Some(Statement::Goto(ast::GotoStmt { label: Some(label) }))
    }

    /// First tok needs to be Token::LCurly
    fn parse_block(&mut self) -> Option<BlockStmt<'a>> {
        let mut block = Vec::new();
        self.next_tok();
        while self.cur_tok() != Some(&Token::RCurly) {
            block.push(self.parse_stmt()?);
        }

        Some(BlockStmt { block })
    }

    fn tok_to_bin_op(tok: &Token<'a>) -> Option<BinOperator> {
        match tok {
            Token::Plus => Some(BinOperator::Add),
            Token::Minus => Some(BinOperator::Sub),
            Token::Asterisk => Some(BinOperator::Mul),
            Token::Divide => Some(BinOperator::Div),
            Token::Mod => Some(BinOperator::Mod),
            Token::LeftShift => Some(BinOperator::LSh),
            Token::RightShift => Some(BinOperator::RSh),
            Token::Ampersand => Some(BinOperator::BAnd),
            Token::BOr => Some(BinOperator::BOr),
            Token::XOr => Some(BinOperator::BXor),
            _ => todo!(),
        }
    }

    fn tok_to_pre_op(&mut self) -> Option<PreOperator<'a>> {
        match self.cur_tok() {
            Some(tok) => match tok {
                Token::Plus => Some(PreOperator::Pos),
                Token::Minus => Some(PreOperator::Neg),
                Token::ExclamMark => Some(PreOperator::Not),
                Token::Not => Some(PreOperator::BNot),
                Token::Asterisk => Some(PreOperator::Deref),
                Token::Sizeof => Some(PreOperator::SizeOf),
                Token::Ampersand => Some(PreOperator::AddrOf),
                Token::LParent => {
                    self.next_tok();
                    let _type = Type::Ident(match self.cur_tok() {
                        Some(Token::Ident(ident)) => ident,
                        _ => todo!("Implement support for more complicated types when casting"),
                    });
                    self.next_tok();
                    Some(PreOperator::Cast(_type))
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
            tok => Precedence::Lowest,
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

    fn expr_to_stmt(&mut self, expr: Expression<'a>) -> Statement<'a> {
        if let Some(Token::Semicolon) = self.peek_tok() {
            self.next_tok();
            self.next_tok();
        }
        Statement::Expression(expr)
    }
}
