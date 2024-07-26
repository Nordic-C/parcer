use logos::Logos;

use self::tokens::Token;

pub mod tokens;

pub struct Lexer<'a> {
    pub input: &'a str,
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let tokens = Token::lexer(input)
            .map(|tok| match tok {
                Ok(Token::LitString(str)) => Token::LitString(trim_str_tok(str)),
                Ok(tok) => tok,
                Err(_) => panic!(),
            })
            .collect();
        Self { input, tokens }
    }
}

fn trim_str_tok(str: &str) -> &str {
    &str[1..str.len()-1]
}
