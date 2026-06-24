use crate::utils::errors::ShellErrs;
use std::io::{self, Write};

pub static USAGE: &str = "
Usage: echo [-n] [text ...]
Print text to standard output.
Options:
    -n    Do not print the trailing newline
";

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    let (newline, text_args) = match args.first() {
        Some(flag) if flag == "-n" => (false, &args[1..]),
        _ => (true, args),
    };

    print!("{}", text_args.join(" "));
    if newline {
        println!();
    } else {
        io::stdout()
            .flush()
            .map_err(|e| ShellErrs::general(&format!("failed to flush stdout: {}", e)))?;
    }

    Ok(())
}
