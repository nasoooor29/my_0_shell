use std::fs;
use std::path::Path;

fn main() {
    let commands_dir = Path::new("src/commands");
    let mut output = String::new();
    let mut imports = String::new();

    for entry in fs::read_dir(commands_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let file = path.file_stem().unwrap().to_str().unwrap();
            if file == "mod" {
                continue;
            }
            output.push_str(&format!(
                "reg.insert(
    \"{file}\".to_string(),
    Command {{
        name: \"{file}\".to_string(),
        func: {file}::run as CmdFn,
        usage: {file}::USAGE.to_string(),
        }},
    );\n"
            ));
            imports.push_str(&format!("pub mod {};\n", file));
        }
    }

    let final_output = format!(
        "use std::collections::HashMap;

{imports}
use crate::utils::errors::ShellErrs;

pub type CmdFn = fn(&[String]) -> Result<(), ShellErrs>;

pub struct Command {{
    pub name: String,
    pub func: CmdFn,
    pub usage: String,
}}

pub fn init_registry() -> HashMap<String, Command> {{
    let mut reg = HashMap::new();

{output}
    return reg;
}}"
    );
    fs::write("./src/commands/mod.rs", final_output).unwrap();
}
