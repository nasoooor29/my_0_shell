use crate::{define_options, utils::errors::ShellErrs};
use std::fs::File;
use std::io::{self, Write};

pub static USAGE: &str = "
Usage: cat file ...
Print file contents to standard output.
";

define_options!(CatOptions {
    flags: {},
    positional: files,
    default_positional: "",
});

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    let options = CatOptions::parse(args)?;
    if options.files[0].is_empty() {
        return Err(ShellErrs::invalid_number_of_arguments(1, 0));
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for file in options.files {
        let mut input =
            File::open(&file).map_err(|e| ShellErrs::general(&format!("{}: {}", file, e)))?;

        io::copy(&mut input, &mut handle)
            .map_err(|e| ShellErrs::general(&format!("{}: {}", file, e)))?;

        println!(); // Print a newline after each file's contents
    }

    handle
        .flush()
        .map_err(|e| ShellErrs::general(&format!("failed to flush stdout: {}", e)))?;

    Ok(())
}
