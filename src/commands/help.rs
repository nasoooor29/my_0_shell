use crate::{commands::init_registry, utils::errors::ShellErrs};
pub static USAGE: &str = "
    help
";
pub fn run(args: &[String]) -> Result<(), ShellErrs> {
    if args.len() != 0 {
        return Err(ShellErrs::InvalidNumberOfArguments {
            expected: 0,
            found: args.len(),
        });
    }

    let cmds = init_registry();
    for (name, cmd) in cmds.iter() {
        println!("=============[{}]=============\nUsage:{}", name, cmd.usage);
    }

    return Ok(());
}
