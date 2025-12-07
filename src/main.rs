use std::io::{self, Write};

use crate::utils::errors::print_red;

pub mod commands;
pub mod utils;

fn main() {
    // Clear terminal
    print!("\x1B[2J\x1B[H");
    let reg = commands::init_registry();
    println!("Welcome to the idk shit shell! Type 'exit' or 'quit' to leave.");
    println!("there is {} commands avaliable", reg.len());
    println!("type `help` to get the avaliable commands");

    loop {
        print!("{}", "oh lord: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let data = match io::stdin().read_line(&mut input) {
            Ok(b) => b,
            Err(_) => {
                println!("Error reading input. Please try again.");
                continue;
            }
        };
        if data == 0 {
            break;
        }
        if input.trim().is_empty() {
            continue;
        }
        if input.trim().eq_ignore_ascii_case("exit") || input.trim().eq_ignore_ascii_case("quit") {
            break;
        }
        let args = utils::parser::parse_args(&input);
        if args.len() == 0 {
            println!("wtf bro");
            continue;
        }
        let command_name = &args[0];
        let cmd = match reg.get(command_name) {
            None => {
                print_red(format_args!(
                    "Unknown command: '{command_name}'. \nType 'help' for a list of commands."
                ));
                continue;
            }
            Some(c) => c,
        };
        let real_args = &args[1..];
        let res = (cmd.func)(&real_args);
        if res.is_ok() {
            continue;
        }
        let e = res.unwrap_err();
        e.print(&cmd.name, &cmd.usage);
    }
}
