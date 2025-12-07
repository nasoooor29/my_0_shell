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
