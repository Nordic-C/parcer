pub mod tokens;

use std::{io, path::PathBuf};

use clutils::files::FileHandler;
use logos::Logos;

use self::tokens::Token;

pub struct Lexer {
    pub file: FileHandler,
}

impl Lexer {
    pub fn new(file_path: PathBuf) -> io::Result<Self> {
        Ok(Self {
            file: FileHandler::new(file_path)?,
        })
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        for result in Token::lexer(&self.file.file_content) {
            match result {
                Ok(token) => tokens.push(token),
                Err(_) => panic!("some error occurred"),
            }
        }
        tokens
    }
}
