#[derive(Debug)]
pub enum ShellErrs {
    General(String),
    InvalidNumberOfArguments { expected: usize, found: usize },
    InvalidFlag { flag: String },
}

pub struct Printer;

impl Printer {
    pub fn red(args: std::fmt::Arguments) {
        eprint!("\x1b[31m{}\x1b[0m\n", args);
    }
    pub fn green(args: std::fmt::Arguments) {
        eprint!("\x1b[32m{}\x1b[0m\n", args);
    }
    pub fn yellow(args: std::fmt::Arguments) {
        eprint!("\x1b[33m{}\x1b[0m\n", args);
    }
    pub fn blue(args: std::fmt::Arguments) {
        eprint!("\x1b[34m{}\x1b[0m\n", args);
    }
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
                Printer::red(format_args!(
                    "Error: Invalid number of arguments for command '{}'. Expected {}, found {}.\n",
                    command_name, expected, found
                ));
                Printer::red(format_args!("Usage:\n{}\n", usage));
            }

            ShellErrs::InvalidFlag { flag } => {
                Printer::red(format_args!(
                    "Error: Invalid flag '{}' for command '{}'.\n",
                    flag, command_name
                ));
                Printer::red(format_args!("Usage:\n{}\n", usage));
            }

            ShellErrs::General(msg) => {
                Printer::red(format_args!(
                    "An error occurred while executing command '{}': {}\n",
                    command_name, msg
                ));
            }
        }
    }
}
