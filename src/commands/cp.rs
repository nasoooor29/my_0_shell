use crate::{define_options, utils::errors::ShellErrs};
use std::fs;
use std::path::Path;

pub static USAGE: &str = "
Usage: cp source ... target
Copy files.
";

define_options!(CpOptions {
    flags: {},
    positional: paths,
    default_positional: "",
});

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    let options = CpOptions::parse(args)?;
    if options.paths[0].is_empty() {
        return Err(ShellErrs::invalid_number_of_arguments(2, 0));
    }
    if options.paths.len() < 2 {
        return Err(ShellErrs::invalid_number_of_arguments(
            2,
            options.paths.len(),
        ));
    }

    let target = options.paths.last().unwrap();
    let sources = &options.paths[..options.paths.len() - 1];
    let target_path = Path::new(target);

    if sources.len() > 1 && !target_path.is_dir() {
        return Err(ShellErrs::general(&format!(
            "target '{}' is not a directory",
            target
        )));
    }

    for source in sources {
        copy_file(source, target_path)?;
    }

    Ok(())
}

fn copy_file(source: &str, target: &Path) -> Result<(), ShellErrs> {
    let source_path = Path::new(source);
    if source_path.is_dir() {
        return Err(ShellErrs::general(&format!(
            "cannot copy '{}': Is a directory",
            source
        )));
    }

    let destination = if target.is_dir() {
        let file_name = source_path.file_name().ok_or_else(|| {
            ShellErrs::general(&format!("cannot copy '{}': missing file name", source))
        })?;

        target.join(file_name)
    } else {
        target.to_path_buf()
    };

    fs::copy(source_path, &destination)
        .map(|_| ())
        .map_err(|e| {
            ShellErrs::general(&format!(
                "cannot copy '{}' to '{}': {}",
                source,
                destination.display(),
                e
            ))
        })
}
