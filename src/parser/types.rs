use crate::lexer::tokens::Token;

use super::{Parser, Type};

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
                while let Token::Asterisk = self.peek_tok()? {
                    // Restrict and const
                    self.next_tok();
                    let type_ref = self.arena.alloc(type_);
                    type_ = Type::Pointer {
                        type_: type_ref,
                        is_const: false,
                        is_restricted: false,
                    };
                    while let Token::Asterisk | Token::Restrict | Token::Const = self.peek_tok()? {
                        self.next_tok();
                        match self.cur_tok()? {
                            Token::Asterisk => {
                                type_ = Type::Pointer {
                                    type_: self.arena.alloc(type_),
                                    is_const: false,
                                    is_restricted: false,
                                }
                            }
                            Token::Restrict => {
                                if let Type::Pointer { is_restricted, .. } = &mut type_ {
                                    *is_restricted = true;
                                }
                            }
                            Token::Const => {
                                if let Type::Pointer { is_const, .. } = &mut type_ {
                                    *is_const = true;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                Some(type_)
            }
            tok => panic!("Cannot parse type from token: {tok:?}"),
        }
    }
}
