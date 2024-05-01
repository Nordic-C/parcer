#[cfg(test)]
mod tests {
    use bumpalo::Bump;
    use clutils::files::FileHandler;

    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_lexer() {
        let binding = "tests/main.c".into();
        let fh = FileHandler::new(&binding).unwrap();
        let lexer = Lexer::new(&fh);
        println!("{:?}", lexer.tokens)
    }

    #[test]
    fn test_parser() {
        let binding = "tests/main.c".into();
        let fh = FileHandler::new(&binding).unwrap();
        let lexer = Lexer::new(&fh);
        let parse_arena = Bump::new();
        let mut parser = Parser::new(lexer, &parse_arena);
        let stmt = parser.parse_stmt();
        dbg!(stmt);
    }
}
