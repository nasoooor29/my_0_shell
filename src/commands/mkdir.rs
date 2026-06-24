use crate::utils::errors::ShellErrs;
use std::fs;

pub static USAGE: &str = "
Usage: mkdir [-p] directory ...
Create one or more directories.
Options:
    -p    Create parent directories as needed and ignore existing directories
";

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    let mut create_parents = false;
    let mut dirs = Vec::new();

    for arg in args {
        if arg == "-p" {
            create_parents = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            return Err(ShellErrs::invalid_flag(arg));
        } else {
            dirs.push(arg);
        }
    }

    if dirs.is_empty() {
        return Err(ShellErrs::invalid_number_of_arguments(1, 0));
    }

    for dir in dirs {
        let result = if create_parents {
            fs::create_dir_all(dir)
        } else {
            fs::create_dir(dir)
        };

        result.map_err(|e| {
            ShellErrs::general(&format!("cannot create directory '{}': {}", dir, e))
        })?;
    }

    Ok(())
}
