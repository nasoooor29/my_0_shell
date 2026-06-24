use crate::utils::errors::ShellErrs;

pub static USAGE: &str = "
    exit
";

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    if !args.is_empty() {
        return Err(ShellErrs::InvalidNumberOfArguments {
            expected: 0,
            found: args.len(),
        });
    }

    std::process::exit(0);
}
