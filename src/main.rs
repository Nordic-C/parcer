use std::fs;
use bumpalo::Bump;
use parcer::lexer::Lexer;
use parcer::parser::Parser;

fn main() {
    const TESTS_PATH: &str = "tests/main.c";

    print!("Test");
    let file_content = fs::read_to_string(TESTS_PATH).unwrap();
    let lexer = Lexer::new(&file_content);
    let parse_arena = Bump::new();
    let mut parser = Parser::new(lexer, &parse_arena);
    let stmts = parser.parse();
    dbg!(stmts);
}