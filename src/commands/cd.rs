use crate::utils::errors::ShellErrs;
use std::env;

pub static USAGE: &str = "
Usage: cd [directory]
Change the current working directory. With no directory, change to $HOME.
";

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    if args.len() > 1 {
        return Err(ShellErrs::invalid_number_of_arguments(1, args.len()));
    }

    let target = match args.first() {
        Some(path) => path.clone(),
        None => env::var("HOME").map_err(|_| ShellErrs::general("HOME not set"))?,
    };

    env::set_current_dir(&target)
        .map_err(|e| ShellErrs::general(&format!("{}: {}", target, e)))
}
