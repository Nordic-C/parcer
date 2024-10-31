#[cfg(test)]
mod tests {
    use std::fs::{self};

    use bumpalo::Bump;

    use crate::{lexer::Lexer, parser::Parser};

    const TESTS_PATH: &str = "tests/main.c";

    #[test]
    fn test_lexer() {
        let file_content = fs::read_to_string(TESTS_PATH).unwrap();
        let lexer = Lexer::new(&file_content);
        println!("{:?}", lexer.tokens)
    }

    #[test]
    fn test_parser() {
        let file_content = fs::read_to_string(TESTS_PATH).unwrap();
        let lexer = Lexer::new(&file_content);
        let parse_arena = Bump::new();
        let mut parser = Parser::new(lexer, &parse_arena);
        let stmts = parser.parse();
        dbg!(stmts);
    }

    #[test]
    fn test_src_code_reconstruction() {
        let file_content = fs::read_to_string(TESTS_PATH).unwrap();
        let lexer = Lexer::new(&file_content);
        let parse_arena = Bump::new();
        let mut parser = Parser::new(lexer, &parse_arena);
        let stmts = parser.parse();
    }

}
