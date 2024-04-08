#[macro_export]
macro_rules! expect_peek {
    ($tok:expr,$pat:pat,$fail:expr) => {{
        if !matches!($tok, $pat) {
            $fail($tok);
            false
        } else {
            true
        }
    }};
}

#[macro_export]
macro_rules! parser_error {
    ($($arg:tt)+) => {{
        use colored::Colorize;

        eprintln!("{}: {}", "Parser Error".red(), format_args!($($arg)+))
    }};
}

#[macro_export]
macro_rules! valid_var_or_func {
    () => {
        Some(Token::Const)
            | Some(Token::Static)
            | Some(Token::Register)
            | Some(Token::Inline)
            | Some(Token::Volatile)
    };
}
