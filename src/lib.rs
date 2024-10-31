use ast::stmt::Statement;

pub mod ast;
pub mod lexer;
pub mod parser;
mod tests;

pub fn ast_to_string(ast: Vec<Statement<'_>>) -> String {
    let mut str = String::new();
    for stmt in ast {
        str.push_str(&stmt.to_string());
    }
    str
}
