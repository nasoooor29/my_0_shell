use std::io::{self, Read, Write};
use std::process::Command;

/// Raw terminal mode RAII guard using stty
pub struct RawMode {
    original_settings: String,
}

impl RawMode {
    pub fn enable() -> io::Result<Self> {
        // Save current terminal settings
        let output = Command::new("stty").arg("-g").output()?;

        let original_settings = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Enable raw mode: disable canonical mode and echo
        Command::new("stty").args(["-icanon", "-echo"]).status()?;

        Ok(Self { original_settings })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        // Restore original terminal settings
        let _ = Command::new("stty").arg(&self.original_settings).status();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Key {
    Char(char),
    Ctrl(char), // Ctrl + letter (A–Z, a–z)
    Enter,
    Backspace,
    Arrow(Direction),
    Esc,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_escape_sequence() -> io::Result<Key> {
    let mut seq = [0u8; 2];

    // If we can't read more bytes, it's a plain Esc key
    if io::stdin().read_exact(&mut seq).is_err() {
        return Ok(Key::Esc);
    }

    if seq[0] != b'[' {
        return Ok(Key::Esc);
    }

    Ok(match seq[1] {
        b'A' => Key::Arrow(Direction::Up),
        b'B' => Key::Arrow(Direction::Down),
        b'C' => Key::Arrow(Direction::Right),
        b'D' => Key::Arrow(Direction::Left),
        _ => Key::Unknown,
    })
}

/// Read a single key from stdin
pub fn read_key() -> io::Result<Key> {
    let mut buf = [0u8; 1];
    io::stdin().read_exact(&mut buf)?;
    let b = buf[0];

    Ok(match b {
        // Enter
        b'\n' | b'\r' => Key::Enter,

        // Backspace (DEL or BS)
        127 | 8 => Key::Backspace,

        // Ctrl+A (1) .. Ctrl+Z (26)
        1..=26 => Key::Ctrl((b + b'a' - 1) as char),

        // Escape or escape sequence
        27 => read_escape_sequence()?,

        // Printable ASCII
        c if (32..127).contains(&c) => Key::Char(c as char),

        _ => Key::Unknown,
    })
}

/// Read a line with history support
pub fn read_line(
    prompt: &str,
    history: &[String],
    history_cursor: &mut usize,
) -> io::Result<Option<String>> {
    let _raw = RawMode::enable()?;

    let mut input = String::new();
    let mut cursor_pos = 0;
    let mut temp_input = String::new();

    print!("{}", prompt);
    io::stdout().flush()?;

    loop {
        match read_key()? {
            Key::Enter => {
                println!();
                return Ok(Some(input));
            }
            Key::Ctrl('c') => {
                println!("^C");
                input.clear();
                cursor_pos = 0;
                print!("{}", prompt);
                io::stdout().flush()?;
            }
            Key::Ctrl('d') => {
                if input.is_empty() {
                    println!();
                    return Ok(None); // EOF
                }
            }
            Key::Backspace => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                    input.remove(cursor_pos);
                    redraw_line(prompt, &input, cursor_pos)?;
                }
            }
            Key::Arrow(Direction::Up) => {
                if !history.is_empty() && *history_cursor > 0 {
                    if *history_cursor == history.len() {
                        temp_input = input.clone();
                    }
                    *history_cursor -= 1;
                    input = history[*history_cursor].clone();
                    cursor_pos = input.len();
                    redraw_line(prompt, &input, cursor_pos)?;
                }
            }
            Key::Arrow(Direction::Down) => {
                if *history_cursor < history.len() {
                    *history_cursor += 1;
                    if *history_cursor == history.len() {
                        input = temp_input.clone();
                    } else {
                        input = history[*history_cursor].clone();
                    }
                    cursor_pos = input.len();
                    redraw_line(prompt, &input, cursor_pos)?;
                }
            }
            Key::Arrow(Direction::Left) => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                    print!("\x1b[D");
                    io::stdout().flush()?;
                }
            }
            Key::Arrow(Direction::Right) => {
                if cursor_pos < input.len() {
                    cursor_pos += 1;
                    print!("\x1b[C");
                    io::stdout().flush()?;
                }
            }
            Key::Char(c) => {
                input.insert(cursor_pos, c);
                cursor_pos += 1;
                if cursor_pos == input.len() {
                    print!("{}", c);
                    io::stdout().flush()?;
                } else {
                    redraw_line(prompt, &input, cursor_pos)?;
                }
            }
            Key::Esc => {
                // Ignore escape key
            }
            Key::Unknown => {
                println!("\nUnknown key pressed");
            }
            _ => {
                // Ignore other keys
            }
        }
    }
}

fn redraw_line(prompt: &str, input: &str, cursor_pos: usize) -> io::Result<()> {
    print!("\r\x1b[K{}{}", prompt, input);
    let back = input.len() - cursor_pos;
    if back > 0 {
        print!("\x1b[{}D", back);
    }
    io::stdout().flush()
}
