use crate::{define_options, utils::errors::ShellErrs};
use std::fs;
use std::path::Path;

pub static USAGE: &str = "
Usage: rm [-r] file ...
Remove files or directories.
Options:
    -r    Remove directories and their contents recursively
";

define_options!(RmOptions {
    flags: {
        'r' => recursive,
    },
    positional: paths,
    default_positional: "",
});

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    let options = RmOptions::parse(args)?;
    if options.paths[0].is_empty() {
        return Err(ShellErrs::invalid_number_of_arguments(1, 0));
    }

    for path in options.paths {
        remove_path(&path, options.recursive)?;
    }

    Ok(())
}

fn remove_path(path: &str, recursive: bool) -> Result<(), ShellErrs> {
    let path_ref = Path::new(path);
    let metadata = fs::symlink_metadata(path_ref)
        .map_err(|e| ShellErrs::general(&format!("cannot remove '{}': {}", path, e)))?;

    let result = if metadata.file_type().is_dir() {
        if !recursive {
            return Err(ShellErrs::general(&format!(
                "cannot remove '{}': Is a directory",
                path
            )));
        }
        fs::remove_dir_all(path_ref)
    } else {
        fs::remove_file(path_ref)
    };

    result.map_err(|e| ShellErrs::general(&format!("cannot remove '{}': {}", path, e)))
}
