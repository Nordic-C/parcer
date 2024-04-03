#[cfg(test)]
mod tests {
    use std::{fs, vec};

    use crate::lexer::{tokens::Token, Lexer};

    fn tokenize_and_assert(input: &str, expected: Vec<Token>) {
        let mut lexer = Lexer::new(input.into());
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.tokenize();
            if tok == Token::Eof {
                break;
            }
            tokens.push(tok);
        }
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_keywords() {
        let input = "static void main int struct union";
        let expected = vec![
            Token::Static,
            Token::Void,
            Token::Ident("main".into()),
            Token::Int,
            Token::Struct,
            Token::Union,
        ];
        tokenize_and_assert(input, expected);
    }

    #[test]
    fn test_numbers() {
        let input = "100.0 20 3000 400 .80";
        let expected = vec![
            Token::LitFloat(100.0),
            Token::LitInt(20),
            Token::LitInt(3000),
            Token::LitInt(400),
            Token::LitFloat(0.80),
        ];
        tokenize_and_assert(input, expected);
    }

    #[test]
    fn test_strings() {
        let input = r#""hello" "world""#;
        let expected = vec![
            Token::LitString("hello".into()),
            Token::LitString("world".into()),
        ];
        tokenize_and_assert(input, expected);
    }

    #[test]
    fn test_src_file() {
        let input = fs::read_to_string("tests/main.c").unwrap();
        let expected = vec![
            Token::Int,
            Token::Ident("main".into()),
            Token::LParent,
            Token::Int,
            Token::Ident("argc".into()),
            Token::Comma,
            Token::Char,
            Token::Asterisk,
            Token::Asterisk,
            Token::Ident("argv".into()),
            Token::RParent,
            Token::LCurly,
            Token::Eol,
            Token::Float,
            Token::Ident("x".into()),
            Token::Assign,
            Token::LitFloat(0.02 as f64),
            Token::Semicolon,
            Token::Eol,
            Token::RCurly,
        ];
        tokenize_and_assert(&input, expected)
    }
}
