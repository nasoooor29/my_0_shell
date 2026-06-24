use crate::{define_options, utils::errors::ShellErrs};
use std::io::{self, Write};

pub static USAGE: &str = "
Usage: echo [-n] [text ...]
Print text to standard output.
Options:
    -n    Do not print the trailing newline
";

define_options!(EchoOptions {
    flags: {
        'n' => no_newline,
    },
    positional: args,
    default_positional: "",
});

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    let options = EchoOptions::parse(args)?;

    println!("{}", options.args.join(" "));
    if options.no_newline {
        io::stdout()
            .flush()
            .map_err(|e| ShellErrs::general(&format!("failed to flush stdout: {}", e)))?;
    }

    Ok(())
}
