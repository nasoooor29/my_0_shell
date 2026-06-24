use crate::{define_options, utils::errors::ShellErrs};
use std::fs;
use std::path::Path;

pub static USAGE: &str = "
Usage: mv source ... target
Move or rename files and directories.
";

define_options!(MvOptions {
    flags: {},
    positional: paths,
    default_positional: "",
});

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    let options = MvOptions::parse(args)?;
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
        move_path(source, target_path)?;
    }

    Ok(())
}

fn move_path(source: &str, target: &Path) -> Result<(), ShellErrs> {
    let source_path = Path::new(source);
    let destination = if target.is_dir() {
        let file_name = source_path.file_name().ok_or_else(|| {
            ShellErrs::general(&format!("cannot move '{}': missing file name", source))
        })?;

        target.join(file_name)
    } else {
        target.to_path_buf()
    };

    fs::rename(source_path, &destination).map_err(|e| {
        ShellErrs::general(&format!(
            "cannot move '{}' to '{}': {}",
            source,
            destination.display(),
            e
        ))
    })
}
