#[cfg(test)]
mod tests {
    use std::io;

    use crate::lexer::Lexer;

    #[test]
    fn test_lexer() -> io::Result<()> {
        let lexer = Lexer::new("tests/main.c".into())?;
        let tokens = lexer.tokenize();
        println!("{tokens:?}");
        Ok(())
    }
}
