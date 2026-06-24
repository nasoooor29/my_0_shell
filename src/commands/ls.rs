use crate::define_options;
use crate::utils::errors::ShellErrs;
use crate::utils::format::{self, colors};
use std::fs;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::Path;

pub static USAGE: &str = "
    ls [options] [paths]
Options:
    -a    Show all files, including hidden files
    -l    Use a long listing format
    -h    Show file sizes in human-readable format
    -F    Append indicator (one of */=>@|) to entries
";

define_options!(LsOptions {
    flags: {
        'a' => show_all,
        'l' => long_format,
        'h' => human_readable,
        'F' => classify,
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

        let display_name = format_display_name(&name, path, Some(&metadata), options.classify);

        println!(
            "{} {:>3} {:>5} {:>5} {} {} {}",
            perms,
            nlink,
            uid,
            gid,
            size,
            modified,
            colors::colorize_file(&display_name, Some(&metadata))
        );
    } else {
        let metadata = path.metadata().ok();
        let display_name = format_display_name(&name, path, metadata.as_ref(), options.classify);
        print!(
            "{}  ",
            colors::colorize_file(&display_name, metadata.as_ref())
        );
    }

    Ok(())
}

fn format_display_name(
    name: &str,
    path: &Path,
    metadata: Option<&fs::Metadata>,
    classify: bool,
) -> String {
    if !classify {
        return name.to_string();
    }

    let link_metadata = path.symlink_metadata().ok();
    let metadata = link_metadata.as_ref().or(metadata);

    format!("{}{}", name, metadata.map(classify_indicator).unwrap_or(""))
}

fn classify_indicator(metadata: &fs::Metadata) -> &'static str {
    let file_type = metadata.file_type();

    if file_type.is_dir() {
        "/"
    } else if file_type.is_symlink() {
        "@"
    } else if file_type.is_fifo() {
        "|"
    } else if file_type.is_socket() {
        "="
    } else if metadata.permissions().mode() & 0o111 != 0 {
        "*"
    } else {
        ""
    }
}
