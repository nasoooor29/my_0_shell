#[derive(Debug)]
pub enum ShellErrs {
    General(String),
    InvalidNumberOfArguments { expected: usize, found: usize },
    InvalidFlag { flag: String },
}

/// Print red-colored text using `format_args!`
pub fn print_red(args: std::fmt::Arguments) {
    eprint!("\x1b[31m{}\x1b[0m\n", args);
}

impl ShellErrs {
    pub fn general(msg: &str) -> Self {
        ShellErrs::General(msg.to_string())
    }

    pub fn invalid_number_of_arguments(expected: usize, found: usize) -> Self {
        ShellErrs::InvalidNumberOfArguments { expected, found }
    }

    pub fn invalid_flag(flag: &str) -> Self {
        ShellErrs::InvalidFlag {
            flag: flag.to_string(),
        }
    }

    pub fn print(&self, command_name: &str, usage: &str) {
        match self {
            ShellErrs::InvalidNumberOfArguments { expected, found } => {
                print_red(format_args!(
                    "Error: Invalid number of arguments for command '{}'. Expected {}, found {}.\n",
                    command_name, expected, found
                ));
                print_red(format_args!("Usage:\n{}\n", usage));
            }

            ShellErrs::InvalidFlag { flag } => {
                print_red(format_args!(
                    "Error: Invalid flag '{}' for command '{}'.\n",
                    flag, command_name
                ));
                print_red(format_args!("Usage:\n{}\n", usage));
            }

            ShellErrs::General(msg) => {
                print_red(format_args!(
                    "An error occurred while executing command '{}': {}\n",
                    command_name, msg
                ));
            }
        }
    }
}
