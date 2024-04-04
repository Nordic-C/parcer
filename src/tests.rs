#[cfg(test)]
mod tests {
    use clutils::files::FileHandler;

    use crate::lexer::Lexer;

    #[test]
    fn test_lexer() {
        let fh = FileHandler::new("tests/main.c".into()).unwrap();
        let lexer = Lexer::new(&fh);
        println!("{:?}", lexer.tokens)
    }
}
