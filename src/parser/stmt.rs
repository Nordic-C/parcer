use crate::{ast::{stmt::{CompositeDataType, DataStorageClass, Field, FunctionStmt, VariableStmt}, types::Type}, encounter_dsc_modifier, encounter_modifier, expect_tok, lexer::tokens::Token, parser_error};

use super::{BlockStmt, Parser, Statement};

impl<'a, 's: 'a> Parser<'a, 's> {
    pub fn parse_stmt(&mut self) -> Option<Statement<'a>> {
        match self.cur_tok()? {
            Token::Ident(_) => self.parse_ident(),
            Token::Auto | Token::Const | Token::Register => self.parse_variable(),
            Token::Static | Token::Volatile | Token::Extern => self.parse_var_or_func(),
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
            Token::Do => todo!(),
            Token::For => todo!(),
            Token::While => todo!(),
            Token::Switch => todo!(),
            Token::Typedef => todo!(),
            Token::Semicolon => {
                self.next_tok();
                self.parse_stmt()
            }
            Token::LCurly => self.parse_block().map(|block| Statement::Block(block)),
            _ => self.parse_expr().map(|expr| Statement::Expression(expr)),
        }
    }

    pub(super) fn parse_variable(&mut self) -> Option<Statement<'a>> {
        let mut var_type = if let Token::Ident(ident) = self.cur_tok()? {
            Some(Type::Ident(&ident))
        } else {
            None
        };
        let mut is_const = false;
        let mut is_volatile = false;
        let mut data_storage_class = DataStorageClass::None;
        while self.peek_tok()? != &Token::Assign
            && self.peek_tok()? != &Token::Semicolon
            && self.peek_tok()? != &Token::LSquare
        {
            match *self.cur_tok()? {
                // TODO: Allow multiple const/volatile
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
                Token::Asterisk => {
                    let type_ref = self.arena.alloc(var_type.unwrap());
                    let mut is_restricted = false;
                    let mut is_const = false;
                    while let Token::Restrict | Token::Const = self.peek_tok()? {
                        match self.peek_tok()? {
                            Token::Restrict => is_restricted = true,
                            Token::Const => is_const = true,
                            _ => unreachable!(),
                        }
                        self.next_tok();
                    }
                    var_type = Some(Type::Pointer {
                        type_: type_ref,
                        is_const,
                        is_restricted,
                    });
                }
                //Token::Signed => todo!(),
                //Token::Unsigned => todo!(),
                Token::Enum => var_type = self.encounter_cdt_pointer(CompositeDataType::Enum),
                Token::Struct => var_type = self.encounter_cdt_pointer(CompositeDataType::Struct),
                Token::Union => var_type = self.encounter_cdt_pointer(CompositeDataType::Union),
                Token::Ident(ident) => match var_type {
                    Some(_) => (),
                    None => var_type = Some(Type::Ident(ident)),
                },
                tok => todo!("{tok:?}"),
            }
            self.next_tok();
        }
        let name = match self.cur_tok()? {
            Token::Ident(ident) => *ident,
            _ => unreachable!(),
        };
        if let Token::LSquare = self.peek_tok()? {
            self.next_tok();
            let size: Option<usize> = match *self.peek_tok()? {
                Token::LitInt(int) => {
                    self.next_tok();
                    Some(int.parse().unwrap())
                }
                _ => None,
            };
            let alloc = self.arena.alloc(var_type.unwrap());
            var_type = Some(Type::Array { type_: alloc, size });
            self.next_tok();
        }
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
            parser_error!(
                "Expected semicolon after variable definition, received: {tok:?} instead"
            );
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
            _type: var_type?,
            val: expr,
        }))
    }

    pub(super) fn parse_function(&mut self) -> Option<Statement<'a>> {
        dbg!("Parsing function");
        let mut ret_type = if let Token::Ident(ident) = self.cur_tok()? {
            Some(Type::Ident(&ident))
        } else {
            None
        };
        let mut should_inline = false;
        let mut is_volatile = false;
        let mut data_storage_class = DataStorageClass::None;
        while self.peek_tok()? != &Token::LParent {
            match *self.cur_tok()? {
                Token::Volatile => {
                    encounter_modifier!(is_volatile, "Encountered second `volatile` specification")
                }
                Token::Inline => {
                    encounter_modifier!(should_inline, "Encountered second `inline` specification")
                }
                Token::Static => {
                    encounter_dsc_modifier!(data_storage_class, DataStorageClass::Static)
                }
                Token::Extern => {
                    encounter_dsc_modifier!(data_storage_class, DataStorageClass::Extern)
                }
                Token::Asterisk => {
                    let type_ref = self.arena.alloc(ret_type.unwrap());
                    let is_restricted = if let Token::Restrict = self.peek_tok()? {
                        self.next_tok();
                        true
                    } else {
                        false
                    };
                    let is_const = if let Token::Const = self.peek_tok()? {
                        self.next_tok();
                        true
                    } else {
                        false
                    };
                    ret_type = Some(Type::Pointer {
                        type_: type_ref,
                        is_const,
                        is_restricted,
                    });
                }
                //Token::Signed => todo!(),
                //Token::Unsigned => todo!(),
                Token::Enum => ret_type = self.encounter_cdt_pointer(CompositeDataType::Enum),
                Token::Struct => ret_type = self.encounter_cdt_pointer(CompositeDataType::Struct),
                Token::Union => ret_type = self.encounter_cdt_pointer(CompositeDataType::Union),
                Token::Ident(ident) => match ret_type {
                    Some(_) => (),
                    None => ret_type = Some(Type::Ident(ident)),
                },
                tok => todo!("{tok:?}"),
            }
            self.next_tok();
        }
        let name = match self.cur_tok()? {
            Token::Ident(ident) => *ident,
            _ => unreachable!(),
        };
        expect_tok!(self.peek_tok()?, Token::LParent, |tok| {
            parser_error!(
                "Expected left parenthesis after function name, recevied {tok:?} instead"
            );
        });
        let args = self.parse_field_list(Token::Comma, Token::RParent)?;
        expect_tok!(self.peek_tok()?, Token::RParent, |tok| {
            parser_error!("Expected right parenthesis after arguments, received {tok:?} instead");
        });
        self.next_tok();
        match self.peek_tok()? {
            Token::Semicolon => {
                self.next_tok();
                self.next_tok();
                Some(Statement::Function(FunctionStmt {
                    name,
                    is_volatile,
                    should_inline,
                    data_storage_class,
                    args,
                    ret_type: ret_type?,
                    body: None,
                }))
            }
            Token::LCurly => {
                let func = Some(Statement::Function(FunctionStmt {
                    name,
                    is_volatile,
                    should_inline,
                    data_storage_class,
                    args,
                    ret_type: ret_type?,
                    body: self.parse_block(),
                }));
                self.next_tok();
                self.next_tok();
                func
            }
            tok => {
                parser_error!("Expected semicolon or left curly brackets after function argument parenthesis, received {tok:?} instead");
                panic!()
            }
        }
    }

    /// First token needs to be the token before the first type
    fn parse_field_list(&mut self, seperator: Token<'a>, end: Token<'a>) -> Option<Vec<Field<'a>>> {
        let mut fields: Vec<Field<'a>> = Vec::new();
        self.next_tok();
        // manually parse first field
        dbg!(&end);
        let field = if self.peek_tok()? != &end {
            self.next_tok();
            let mut field_type = self.parse_type()?;
            self.next_tok();
            let name = match *self.cur_tok()? {
                Token::Ident(ident) => ident,
                _ => todo!(),
            };
            if let Token::LSquare = self.peek_tok()? {
                self.next_tok();
                let size: Option<usize> = match *self.peek_tok()? {
                    Token::LitInt(int) => {
                        self.next_tok();
                        Some(int.parse().unwrap())
                    }
                    _ => None,
                };
                let alloc = self.arena.alloc(field_type);
                field_type = Type::Array { type_: alloc, size };
                self.next_tok();
            }
            Field { name, field_type }
        } else {
            return Some(vec![]);
        };
        fields.push(field);
        while self.peek_tok()? == &seperator && self.peek_tok()? != &end {
            self.next_tok();
            self.next_tok();
            let mut type_ = self.parse_type();
            self.next_tok();
            let name = *match self.cur_tok()? {
                Token::Ident(ident) => ident,
                _ => todo!(),
            };
            if let Token::LSquare = self.peek_tok()? {
                self.next_tok();
                let size: Option<usize> = match *self.peek_tok()? {
                    Token::LitInt(int) => {
                        self.next_tok();
                        Some(int.parse().unwrap())
                    }
                    _ => None,
                };
                let alloc = self.arena.alloc(type_.unwrap());
                type_ = Some(Type::Array { type_: alloc, size });
                self.next_tok();
            }
            fields.push(Field {
                name,
                field_type: type_?,
            })
        }
        Some(fields)
    }

    fn parse_block(&mut self) -> Option<BlockStmt<'a>> {
        self.next_tok();
        if let Token::RCurly = self.peek_tok()? {
            return Some(BlockStmt { block: vec![] });
        }

        todo!()
    }

    fn encounter_cdt_pointer(&mut self, _type: CompositeDataType) -> Option<Type<'a>> {
        let name = *match self.peek_tok()? {
            Token::Ident(ident) => ident,
            _ => unreachable!(),
        };
        self.next_tok();
        match _type {
            CompositeDataType::Struct => Some(Type::Struct(name)),
            CompositeDataType::Enum => Some(Type::Enum(name)),
            CompositeDataType::Union => Some(Type::Union(name)),
        }
    }
}
