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
                Ok(Token::LitString(str)) => Token::LitString(trim_str_tok(str)),
                Ok(tok) => tok,
                Err(_) => panic!(),
            })
            .collect();
        Self { fh, tokens }
    }
}

fn trim_str_tok(str: &str) -> &str {
    &str[1..str.len()-1]
}
