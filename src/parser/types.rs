use crate::{ast::types::Type, lexer::tokens::Token, parser_error};

use super::{CompositeDataType, Parser};

impl<'a, 's: 'a> Parser<'a, 's> {
    /// First token needs to be the first token of the type
    pub(super) fn parse_type(&mut self) -> Option<Type<'a>> {
        match self.cur_tok()? {
            Token::Signed => todo!(),
            Token::Unsigned => todo!(),
            Token::Enum => {
                let id = self.id_for_cdt_ptr(CompositeDataType::Enum)?;
                self.next_tok();
                Some(self.parse_ptr(Type::Enum(id))?)
            }
            Token::Struct => {
                let id = self.id_for_cdt_ptr(CompositeDataType::Struct)?;
                self.next_tok();
                Some(self.parse_ptr(Type::Struct(id))?)
            }
            Token::Union => {
                let id = self.id_for_cdt_ptr(CompositeDataType::Union)?;
                self.next_tok();
                Some(self.parse_ptr(Type::Union(id))?)
            }
            Token::Ident(ident) => Some(self.parse_ptr(Type::Ident(ident))?),
            tok => panic!("Cannot parse type from token: {tok:?}"),
        }
    }

    fn id_for_cdt_ptr(&mut self, cdt: CompositeDataType) -> Option<&'a str> {
        Some(match self.peek_tok()? {
            Token::Ident(id) => id,
            _ => {
                parser_error!(
                    "Expected identifier for {cdt} pointer, received {:?} instead",
                    self.peek_tok().unwrap()
                );
                ""
            }
        })
    }

    fn parse_ptr(&mut self, mut type_: Type<'a>) -> Option<Type<'a>> {
        let mut ptr_const = false;
        let mut ptr_restrict = false;
        Some(loop {
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
                _ => break type_,
            }
            self.next_tok();
        })
    }
}
