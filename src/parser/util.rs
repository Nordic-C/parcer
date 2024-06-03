#[macro_export]
macro_rules! expect_tok {
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

        eprintln!("{}: {}", "Parser Error".red(), format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! valid_var_or_func {
    () => {
        Token::Const
            | Token::Static
            | Token::Register
            | Token::Inline
            | Token::Volatile
    };
}

#[macro_export]
macro_rules! encounter_modifier {
    ($var:expr,$msg:expr) => {
        {
            if !$var {
                $var = true;
            } else {
                parser_error!($msg);
            }
        }
    };
}

#[macro_export]
macro_rules! encounter_dsc_modifier {
    ($var:expr,$class:expr) => {
        {
            if let DataStorageClass::None = $var {
                $var = $class;
            } else {
                parser_error!("Encountered second data storage class specifier: {:?}", $class);
            }
        }
    };
}

