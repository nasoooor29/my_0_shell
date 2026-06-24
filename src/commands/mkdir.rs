use crate::{define_options, utils::errors::ShellErrs};
use std::fs;

pub static USAGE: &str = "
Usage: mkdir [-p] directory ...
Create one or more directories.
Options:
    -p    Create parent directories as needed and ignore existing directories
";

define_options!(MkdirOptions {
    flags: {
        'p' => create_parents,
    },
    positional: dirs,
    default_positional: "",
});

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    let options = MkdirOptions::parse(args)?;
    if options.dirs[0].is_empty() {
        return Err(ShellErrs::invalid_number_of_arguments(1, 0));
    }

    for dir in options.dirs {
        let result = if options.create_parents {
            fs::create_dir_all(&dir)
        } else {
            fs::create_dir(&dir)
        };

        result.map_err(|e| {
            ShellErrs::general(&format!("cannot create directory '{}': {}", dir, e))
        })?;
    }

    Ok(())
}
