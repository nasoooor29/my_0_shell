#[macro_export]
macro_rules! define_options {
    (
        $name:ident {
            flags: { $($flag:literal => $field:ident),* $(,)? }
            $(, positional: $pos_field:ident)?
            $(, default_positional: $default:expr)?
            $(,)?
        }
    ) => {
        #[derive(Debug, Default)]
        struct $name {
            $($field: bool,)*
            $($pos_field: Vec<String>,)?
        }

        impl $name {
            fn parse(args: &[String]) -> Result<Self, $crate::utils::errors::ShellErrs> {
                let mut opts = Self::default();
                for arg in args {
                    if arg.starts_with('-') {
                        for ch in arg.chars().skip(1) {
                            match ch {
                                $($flag => opts.$field = true,)*
                                _ => return Err($crate::utils::errors::ShellErrs::invalid_flag(&format!("-{}", ch))),
                            }
                        }
                    } else {
                        $(opts.$pos_field.push(arg.clone());)?
                    }
                }
                $(
                    if opts.$pos_field.is_empty() {
                        opts.$pos_field.push($default.to_string());
                    }
                )?
                Ok(opts)
            }
        }
    };
}

pub fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';

    for c in input.chars() {
        if in_quotes {
            if c == quote_char {
                in_quotes = false;
                args.push(current.trim().to_string());
                current.clear();
            } else {
                current.push(c);
            }
        } else {
            match c {
                '"' | '\'' => {
                    in_quotes = true;
                    quote_char = c;
                }
                c if c.is_whitespace() => {
                    if !current.is_empty() {
                        args.push(current.trim().to_string());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }
    }

    if !current.is_empty() {
        args.push(current.trim().to_string());
    }

    args
}
