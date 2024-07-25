use crate::{ast::types::Type, lexer::tokens::Token};

use super::Parser;

impl<'a, 's: 'a> Parser<'a, 's> {
    /// First token needs to be the first token of the type
    pub(super) fn parse_type(&mut self) -> Option<Type<'a>> {
        match self.cur_tok()? {
            Token::Signed => todo!(),
            Token::Unsigned => todo!(),
            Token::Enum => todo!(),
            Token::Struct => todo!(),
            Token::Union => todo!(),
            Token::Ident(ident) => {
                let mut type_ = Type::Ident(ident);
                // parse pointer

                let mut ptr_const = false;
                let mut ptr_restrict = false;

                loop {
                    match self.peek_tok()? {
                        Token::Const => match &mut type_ {
                            Type::Pointer { is_const, .. } => *is_const = true,
                            _ => ptr_const = true,
                        },
                        Token::Restrict => match &mut type_ {
                            Type::Pointer { is_restricted, .. } => *is_restricted = true,
                            _ => ptr_restrict = true,
                        },
                        Token::Asterisk => {
                            type_ = Type::Pointer {
                                type_: self.arena.alloc(type_),
                                is_const: ptr_const,
                                is_restricted: ptr_restrict,
                            }
                        }
                        _ => break,
                    }
                    self.next_tok();
                }
                Some(type_)
            }
            tok => panic!("Cannot parse type from token: {tok:?}"),
        }
    }
}
