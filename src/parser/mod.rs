use std::collections::HashSet;

use bumpalo::Bump;

use crate::{ast::{stmt::*, *}, lexer::{tokens::Token, Lexer}};

pub mod expr;
pub mod stmt;
pub mod types;
mod macros;

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
        }
        out
    }

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
            Token::Static | Token::Extern | Token::Volatile => self.parse_var_or_func(),
            // Expression
            tok => todo!("{tok:?}"),
        }
    }

    /// Determines whether two identifier declare a variable or a function
    /// [Parser::cur_tok] is the first ident (the type)
    fn parse_var_or_func(&mut self) -> Option<Statement<'a>> {
        // Peek ahead two tokens to determine
        // if it is a variable or function
        // based on whether we encounter
        // lparent, assign or semicolon
        let mut peek_ahead = 2;
        loop {
            match self.lexer.tokens.get(self.tok_index + peek_ahead)? {
                Token::LSquare => break self.parse_variable(),
                Token::LParent => break self.parse_function(),
                Token::Comma => todo!("multi variable"),
                Token::Assign | Token::Semicolon => break self.parse_variable(),
                _ => (),
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

    fn parse_call_expr(&mut self, left: crate::ast::expr::Expression<'a>) -> _ {
        todo!()
    }
}
