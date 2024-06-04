use std::collections::HashSet;

use bumpalo::Bump;

use crate::{ast::{expr::*, stmt::*, types::*, *}, encounter_dsc_modifier, encounter_modifier, expect_tok, lexer::{tokens::Token, Lexer}, parser_error};

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
        }
        out
    }

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

    fn parse_expr(&mut self) -> Option<Expression<'a>> {
        match self.cur_tok()? {
            Token::LitString(str) => Some(Expression::LiteralString(*str)),
            Token::LitInt(int) => Some(Expression::LiteralInt(int.parse().unwrap())),
            Token::LitFloat(float) => Some(Expression::LiteralFloat(float.parse().unwrap())),
            Token::LitChar(char) => Some(Expression::LiteralChar(char.parse().unwrap())),
            Token::Ident(ident) => Some(Expression::Ident(*ident)),
            _ => todo!(),
        }
    }

    fn parse_variable(&mut self) -> Option<Statement<'a>> {
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

    fn parse_function(&mut self) -> Option<Statement<'a>> {
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

    /// First token needs to be the first token of the type
    fn parse_type(&mut self) -> Option<Type<'a>> {
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

    fn parse_block(&mut self) -> Option<BlockStmt<'a>> {
        self.next_tok();
        if let Token::RCurly = self.peek_tok()? {
            return Some(BlockStmt { block: vec![] });
        }

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
            Token::Static | Token::Extern | Token::Volatile => self.parse_var_or_func(),
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
}
