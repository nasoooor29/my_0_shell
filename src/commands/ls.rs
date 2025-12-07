use crate::utils::errors::ShellErrs;

pub static USAGE: &str = "
    ls [options] [paths]
Options:
    -a    Show all files, including hidden files
    -l    Use a long listing format
    -h    Show file sizes in human-readable format
";

struct LsOptions {
    show_all: bool,
    long_format: bool,
    human_readable: bool,
    paths: Vec<String>,
}
fn parse_args(args: &[String]) -> Result<LsOptions, ShellErrs> {
    let mut show_all = false;
    let mut long_format = false;
    let mut human_readable = false;
    let mut paths = Vec::new();

    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => show_all = true,
                    'l' => long_format = true,
                    'h' => human_readable = true,
                    _ => {
                        return Err(ShellErrs::invalid_flag(&format!("-{}", ch)));
                    }
                }
            }
        } else {
            paths.push(arg.clone());
        }
    }

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    Ok(LsOptions {
        show_all,
        long_format,
        human_readable,
        paths,
    })
}

pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    // For now, just demonstrate parsed result — replace with real logic later
    let options = parse_args(args)?;
    println!("ls called with:");
    println!("  show_all       = {}", options.show_all);
    println!("  long_format    = {}", options.long_format);
    println!("  human_readable = {}", options.human_readable);
    println!("  paths          = {:?}", options.paths);

    Ok(())
}
