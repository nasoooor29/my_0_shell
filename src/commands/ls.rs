use crate::define_options;
use crate::utils::errors::ShellErrs;

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
    // For now, just demonstrate parsed result — replace with real logic later
    let options = LsOptions::parse(args)?;
    println!("ls called with:");
    println!("  show_all       = {}", options.show_all);
    println!("  long_format    = {}", options.long_format);
    println!("  human_readable = {}", options.human_readable);
    println!("  paths          = {:?}", options.paths);

    Ok(())
}
