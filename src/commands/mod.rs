use std::collections::HashMap;

pub mod rm;
pub mod ls;
pub mod cat;
pub mod help;
pub mod exit;
pub mod cd;
pub mod mkdir;
pub mod mv;
pub mod echo;
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
            "rm".to_string(),
            Command {
                name: "rm".to_string(),
                func: rm::run as CmdFn,
                usage: rm::USAGE.to_string(),
            },
        );
        reg.insert(
            "ls".to_string(),
            Command {
                name: "ls".to_string(),
                func: ls::run as CmdFn,
                usage: ls::USAGE.to_string(),
            },
        );
        reg.insert(
            "cat".to_string(),
            Command {
                name: "cat".to_string(),
                func: cat::run as CmdFn,
                usage: cat::USAGE.to_string(),
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
            "mkdir".to_string(),
            Command {
                name: "mkdir".to_string(),
                func: mkdir::run as CmdFn,
                usage: mkdir::USAGE.to_string(),
            },
        );
        reg.insert(
            "mv".to_string(),
            Command {
                name: "mv".to_string(),
                func: mv::run as CmdFn,
                usage: mv::USAGE.to_string(),
            },
        );
        reg.insert(
            "echo".to_string(),
            Command {
                name: "echo".to_string(),
                func: echo::run as CmdFn,
                usage: echo::USAGE.to_string(),
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
