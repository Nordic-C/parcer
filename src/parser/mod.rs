use bumpalo::Bump;

use crate::{
    encounter_dsc_modifier, encounter_modifier, expect_tok,
    lexer::{tokens::Token, Lexer},
    parser::ast::{ContinueStmt, LabelStmt, PointerRestriction, Type},
    parser_error, valid_var_or_func,
};

use self::ast::{
    BinOpExpr, BinOperator, BlockStmt, BreakStmt, CallExpr, CompositeDataType, DataStorageClass,
    Expression, FunctionStmt, Ident, PreOperator, PrefixExpr, Statement, VariableStmt,
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
            out.push(stmt);
            dbg!("aaaaa");
        }
        out
    }

    pub fn parse_stmt(&mut self) -> Option<Statement<'a>> {
        match self.cur_tok()? {
            Token::Ident(ident) => self.parse_ident(),
            Token::Auto => self.parse_variable(),
            Token::Const => self.parse_variable(),
            Token::Static => self.parse_var_or_func(),
            Token::Register => self.parse_variable(),
            Token::Volatile => self.parse_var_or_func(),
            Token::Restrict => todo!(),
            Token::Inline => self.parse_function(),
            Token::Signed => todo!(),
            Token::Unsigned => todo!(),
            Token::Break => todo!(),
            Token::Continue => todo!(),
            Token::Goto => todo!(),
            Token::Return => todo!(),
            Token::Enum => todo!(),
            Token::Struct => self.parse_var_or_func(),
            Token::Union => todo!(),
            Token::If => todo!(),
            Token::Else => todo!(),
            Token::Do => todo!(),
            Token::For => todo!(),
            Token::While => todo!(),
            Token::Switch => todo!(),
            Token::Case => todo!(),
            Token::Default => todo!(),
            Token::Extern => todo!(),
            Token::Sizeof => todo!(),
            Token::Typedef => todo!(),
            Token::LitString(_) => todo!(),
            Token::LitInt(_) => todo!(),
            Token::LitFloat(_) => todo!(),
            Token::LitChar(_) => todo!(),
            Token::Assign => todo!(),
            Token::AssignAdd => todo!(),
            Token::AssignSub => todo!(),
            Token::AssignMul => todo!(),
            Token::AssignDiv => todo!(),
            Token::AssignMod => todo!(),
            Token::AssignBAnd => todo!(),
            Token::AssignBOr => todo!(),
            Token::AssignXor => todo!(),
            Token::AssignLSh => todo!(),
            Token::AssignRSh => todo!(),
            Token::Equals => todo!(),
            Token::NEquals => todo!(),
            Token::LTEquals => todo!(),
            Token::GTEquals => todo!(),
            Token::LessThan => todo!(),
            Token::GreaterThan => todo!(),
            Token::ExclamMark => todo!(),
            Token::And => todo!(),
            Token::Or => todo!(),
            Token::BOr => todo!(),
            Token::XOr => todo!(),
            Token::Not => todo!(),
            Token::LeftShift => todo!(),
            Token::RightShift => todo!(),
            Token::Plus => todo!(),
            Token::Minus => todo!(),
            Token::Divide => todo!(),
            Token::Mod => todo!(),
            Token::Increment => todo!(),
            Token::Decrement => todo!(),
            Token::Comma => todo!(),
            Token::Semicolon => todo!(),
            Token::Dot => todo!(),
            Token::Arrow => todo!(),
            Token::BackSlash => todo!(),
            Token::Ampersand => todo!(),
            Token::Asterisk => todo!(),
            Token::QuestionMark => todo!(),
            Token::Colon => todo!(),
            Token::LSquare => todo!(),
            Token::RSquare => todo!(),
            Token::LParent => todo!(),
            Token::RParent => todo!(),
            Token::LCurly => todo!(),
            Token::RCurly => todo!(),
        }
    }

    fn parse_expr(&mut self) -> Option<Expression<'a>> {
        match self.cur_tok()? {
            Token::LitString(_) => todo!(),
            Token::LitInt(int) => Some(Expression::LiteralInt(int.parse().unwrap())),
            Token::LitFloat(_) => todo!(),
            Token::LitChar(_) => todo!(),
            Token::Ident(_) => todo!(),
            _ => todo!(),
        }
    }

    fn parse_variable(&mut self) -> Option<Statement<'a>> {
        let mut type_ = if let Token::Ident(ident) = self.cur_tok()? {
            Some(Type::Ident(&ident))
        } else {
            None
        };
        let mut is_const = false;
        let mut is_volatile = false;
        let mut data_storage_class = DataStorageClass::None;
        while self.peek_tok()? != &Token::Assign && self.peek_tok()? != &Token::Semicolon {
            match *self.cur_tok()? {
                Token::Volatile => {
                    encounter_modifier!(is_volatile, "Encountered second `volatile` specification")
                }
                Token::Const => {
                    encounter_modifier!(is_const, "Encountered second `const` specification")
                }
                Token::Auto => {
                    encounter_dsc_modifier!(data_storage_class, DataStorageClass::Auto)
                }
                Token::Static => {
                    encounter_dsc_modifier!(data_storage_class, DataStorageClass::Static)
                }
                Token::Register => {
                    encounter_dsc_modifier!(data_storage_class, DataStorageClass::Register)
                }
                Token::Extern => {
                    encounter_dsc_modifier!(data_storage_class, DataStorageClass::Extern)
                }
                //Token::Signed => todo!(),
                //Token::Unsigned => todo!(),
                Token::Enum => todo!(),
                Token::Struct => todo!(),
                Token::Union => todo!(),
                Token::Ident(ident) => match type_ {
                    Some(_) => (),
                    None => type_ = Some(Type::Ident(ident)),
                },
                tok => todo!("{tok:?}"),
            }
            self.next_tok();
        }
        let name = match self.cur_tok()? {
            Token::Ident(ident) => *ident,
            _ => unreachable!(),
        };
        self.next_tok();
        let expr = match self.cur_tok()? {
            Token::Assign => {
                self.next_tok();
                self.parse_expr()
            }
            Token::Semicolon => None,
            _ => todo!(),
        };
        expect_tok!(self.peek_tok()?, Token::Semicolon, |tok| {
            parser_error!("Expected semicolon after variable definition, received: {tok:?} instead");
        });
        // go to semicolon
        self.next_tok();
        // go to token after semicolon
        self.next_tok();
        Some(Statement::Variable(VariableStmt {
            name,
            is_volatile,
            is_const,
            data_storage_class,
            _type: type_?,
            val: expr,
        }))
    }

    fn parse_function(&mut self) -> Option<Statement<'a>> {
        todo!()
    }

    #[inline(always)]
    fn parse_ident(&mut self) -> Option<Statement<'a>> {
        match self.peek_tok()? {
            // Variable or function
            Token::Ident(_) => self.parse_var_or_func(),
            // Pointer type or multiplication
            Token::Asterisk => self.parse_var_or_func(),
            // variable specific keywords
            Token::Const | Token::Register | Token::Auto => self.parse_variable(),
            // Function specific keywords
            Token::Inline => todo!(),
            // Variable or function specifix keywords
            Token::Static | Token::Extern | Token::Volatile => todo!(),
            // Expression
            tok => todo!("{tok:?}"),
        }
    }

    /// Determines whether two identifier declare a variable or a function
    /// [Parser::cur_tok] is the first ident (the type)
    #[inline(always)]
    fn parse_var_or_func(&mut self) -> Option<Statement<'a>> {
        // Peek ahead two tokens to determine
        // if it is a variable or function
        // based on whether we encounter
        // lparent, assign or semicolon
        let mut peek_ahead = 2;
        loop {
            match self.lexer.tokens.get(self.tok_index + peek_ahead)? {
                Token::LParent => break self.parse_function(),
                Token::Comma => todo!("multi variable"),
                Token::Assign | Token::Semicolon => break self.parse_variable(),
                tok => (),
            }
            peek_ahead += 1;
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

    #[inline(always)]
    fn reset_variables(&mut self) {
        self.variables.clear();
    }
}
