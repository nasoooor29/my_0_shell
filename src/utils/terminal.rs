use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::os::raw::c_int;

const STDIN_FILENO: c_int = 0;
const TCSANOW: c_int = 0;
const ICANON: u32 = 0x0002;
const ECHO: u32 = 0x0008;
const VTIME: usize = 5;
const VMIN: usize = 6;

#[repr(C)]
#[derive(Clone, Copy)]
struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

unsafe extern "C" {
    fn tcgetattr(fd: c_int, termios_p: *mut Termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const Termios) -> c_int;
}

/// Raw terminal mode RAII guard using Unix terminal APIs directly.
pub struct RawMode {
    original_settings: Termios,
}

impl RawMode {
    pub fn enable() -> io::Result<Self> {
        let mut original_settings = MaybeUninit::<Termios>::uninit();

        // SAFETY: tcgetattr writes a Termios struct for a valid file descriptor.
        if unsafe { tcgetattr(STDIN_FILENO, original_settings.as_mut_ptr()) } == -1 {
            return Err(io::Error::last_os_error());
        }

        // SAFETY: tcgetattr succeeded, so the struct was initialized.
        let original_settings = unsafe { original_settings.assume_init() };
        let mut raw_settings = original_settings;

        raw_settings.c_lflag &= !(ICANON | ECHO);
        raw_settings.c_cc[VMIN] = 1;
        raw_settings.c_cc[VTIME] = 0;

        // SAFETY: raw_settings is a valid Termios struct derived from tcgetattr.
        if unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &raw_settings) } == -1 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { original_settings })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        // SAFETY: original_settings was captured from tcgetattr for this terminal.
        let _ = unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &self.original_settings) };
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
