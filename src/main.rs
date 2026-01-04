use crate::utils::{errors::Printer, terminal};

pub mod commands;
pub mod utils;

fn main() {
    // Clear terminal
    print!("\x1B[2J\x1B[H");
    let reg = commands::init_registry();
    println!("Welcome to the idk shit shell! Type 'exit' or 'quit' to leave.");
    println!("there is {} commands avaliable", reg.len());
    println!("type `help` to get the avaliable commands");

    let mut history: Vec<String> = Vec::new();
    // it's used down and passed into read line
    #[allow(unused_assignments)]
    let mut history_cursor = history.len();

    loop {
        history_cursor = history.len();

        let input = match terminal::read_line("oh lord: ", &history, &mut history_cursor) {
            Ok(Some(line)) => line,
            Ok(None) => break, // EOF (Ctrl+D)
            Err(_) => {
                println!("Error reading input. Please try again.");
                continue;
            }
        };

        if input.trim().is_empty() {
            continue;
        }

        if input.trim().eq_ignore_ascii_case("exit") || input.trim().eq_ignore_ascii_case("quit") {
            break;
        }

        // Add to history
        let trimmed = input.trim().to_string();
        if history.last() != Some(&trimmed) {
            history.push(trimmed);
        }

        let args = utils::parser::parse_args(&input);
        if args.is_empty() {
            println!("wtf bro");
            continue;
        }

        let command_name = &args[0];
        let cmd = match reg.get(command_name) {
            None => {
                Printer::red(format_args!(
                    "Unknown command: '{command_name}'. \nType 'help' for a list of commands."
                ));
                continue;
            }
            Some(c) => c,
        };

        let real_args = &args[1..];
        let res = (cmd.func)(real_args);
        if res.is_ok() {
            continue;
        }

        let e = res.unwrap_err();
        e.print(&cmd.name, &cmd.usage);
    }
}
