use clutils::files::FileHandler;
use logos::Logos;

use self::tokens::Token;

pub mod tokens;

pub struct Lexer<'a> {
    pub fh: &'a FileHandler<'a>,
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(fh: &'a FileHandler) -> Self {
        let tokens = Token::lexer(&fh.file_content)
            .map(|tok| match tok {
                Ok(tok) => tok,
                Err(_) => panic!(),
            })
            .collect();
        Self { fh, tokens }
    }
}
