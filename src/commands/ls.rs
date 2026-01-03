use crate::define_options;
use crate::utils::errors::ShellErrs;
use crate::utils::format::{self, colors};
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

pub static USAGE: &str = "
    ls [options] [paths]
Options:
    -a    Show all files, including hidden files
    -l    Use a long listing format
    -h    Show file sizes in human-readable format
";

define_options!(LsOptions {
    flags: {
        'a' => show_all,
        'l' => long_format,
        'h' => human_readable,
    },
    positional: paths,
    default_positional: ".",
});

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    let options = LsOptions::parse(args)?;

    let multiple_paths = options.paths.len() > 1;

    for (i, path) in options.paths.iter().enumerate() {
        if multiple_paths {
            if i > 0 {
                println!();
            }
            println!("{}:", path);
        }
        list_directory(path, &options)?;
    }

    Ok(())
}

fn list_directory(path: &str, options: &LsOptions) -> Result<(), ShellErrs> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(ShellErrs::general(&format!(
            "cannot access '{}': No such file or directory",
            path.display()
        )));
    }

    if path.is_file() {
        print_entry(path, options)?;
        return Ok(());
    }

    let mut entries: Vec<_> = fs::read_dir(path)
        .map_err(|e| {
            ShellErrs::general(&format!(
                "cannot open directory '{}': {}",
                path.display(),
                e
            ))
        })?
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if !options.show_all && name.starts_with('.') {
            continue;
        }

        print_entry(&entry.path(), options)?;
    }

    if !options.long_format {
        println!();
    }

    Ok(())
}

fn print_entry(path: &Path, options: &LsOptions) -> Result<(), ShellErrs> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());

    if options.long_format {
        let metadata = path
            .metadata()
            .map_err(|e| ShellErrs::general(&format!("cannot read metadata: {}", e)))?;

        let perms = format::permissions(metadata.permissions().mode(), metadata.is_dir());
        let nlink = metadata.nlink();
        let uid = metadata.uid();
        let gid = metadata.gid();
        let size = if options.human_readable {
            format::human_size(metadata.len())
        } else {
            format!("{:>8}", metadata.len())
        };
        let modified = format::file_time(metadata.modified().ok());

        println!(
            "{} {:>3} {:>5} {:>5} {} {} {}",
            perms,
            nlink,
            uid,
            gid,
            size,
            modified,
            colors::colorize_file(&name, Some(&metadata))
        );
    } else {
        let metadata = path.metadata().ok();
        print!("{}  ", colors::colorize_file(&name, metadata.as_ref()));
    }

    Ok(())
}
