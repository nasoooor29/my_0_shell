use std::collections::HashMap;

pub mod ls;
pub mod help;
pub mod exit;
pub mod cd;
pub mod pwd;
use crate::utils::errors::ShellErrs;

pub type CmdFn = fn(&[String]) -> Result<(), ShellErrs>;

pub struct Command {
    pub name: String,
    pub func: CmdFn,
    pub usage: String,
}

pub fn init_registry() -> HashMap<String, Command> {
    let mut reg = HashMap::new();

        reg.insert(
            "ls".to_string(),
            Command {
                name: "ls".to_string(),
                func: ls::run as CmdFn,
                usage: ls::USAGE.to_string(),
            },
        );
        reg.insert(
            "help".to_string(),
            Command {
                name: "help".to_string(),
                func: help::run as CmdFn,
                usage: help::USAGE.to_string(),
            },
        );
        reg.insert(
            "exit".to_string(),
            Command {
                name: "exit".to_string(),
                func: exit::run as CmdFn,
                usage: exit::USAGE.to_string(),
            },
        );
        reg.insert(
            "cd".to_string(),
            Command {
                name: "cd".to_string(),
                func: cd::run as CmdFn,
                usage: cd::USAGE.to_string(),
            },
        );
        reg.insert(
            "pwd".to_string(),
            Command {
                name: "pwd".to_string(),
                func: pwd::run as CmdFn,
                usage: pwd::USAGE.to_string(),
            },
        );
    reg
}
