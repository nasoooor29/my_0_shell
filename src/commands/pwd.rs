use crate::utils::errors::ShellErrs;

pub static USAGE: &str = "
Usage: pwd
Print the current working directory.
";

pub fn run(_args: &[String]) -> Result<(), ShellErrs> {
    let cwd = std::env::current_dir().map_err(|e| {
        ShellErrs::general(&format!("Failed to get current working directory: {}", e))
    })?;
    println!("{}", cwd.display());
    return Ok(());
}
