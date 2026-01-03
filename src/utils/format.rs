use std::time::SystemTime;

/// Format bytes as human-readable size (e.g., 1.3K, 4.0M)
pub fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:>5}{}", bytes, UNITS[unit_index])
    } else {
        format!("{:>5.1}{}", size, UNITS[unit_index])
    }
}

/// Format a SystemTime as a date string (ls-style)
pub fn file_time(time: Option<SystemTime>) -> String {
    match time {
        Some(t) => {
            let duration = t.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
            let secs = duration.as_secs() as i64;

            let (year, month, day, hour, min) = unix_to_datetime(secs);

            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;

            let six_months_ago = now - (180 * 24 * 60 * 60);

            if secs > six_months_ago {
                format!("{} {:>2} {:02}:{:02}", month_name(month), day, hour, min)
            } else {
                format!("{} {:>2}  {:>4}", month_name(month), day, year)
            }
        }
        None => "            ".to_string(),
    }
}

fn unix_to_datetime(secs: i64) -> (i64, u32, u32, u32, u32) {
    let days = secs / 86400;
    let time_of_day = secs % 86400;
    let hour = (time_of_day / 3600) as u32;
    let min = ((time_of_day % 3600) / 60) as u32;

    let mut year = 1970;
    let mut remaining_days = days;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let days_in_months: [i64; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 0u32;
    for (i, &days_in_month) in days_in_months.iter().enumerate() {
        if remaining_days < days_in_month {
            month = i as u32 + 1;
            break;
        }
        remaining_days -= days_in_month;
    }

    let day = remaining_days as u32 + 1;

    (year, month, day, hour, min)
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}

/// Format Unix file permissions as a string (e.g., "drwxr-xr-x")
pub fn permissions(mode: u32, is_dir: bool) -> String {
    let file_type = if is_dir { 'd' } else { '-' };

    let user = triplet((mode >> 6) & 0o7);
    let group = triplet((mode >> 3) & 0o7);
    let other = triplet(mode & 0o7);

    format!("{}{}{}{}", file_type, user, group, other)
}

fn triplet(bits: u32) -> String {
    let r = if bits & 0o4 != 0 { 'r' } else { '-' };
    let w = if bits & 0o2 != 0 { 'w' } else { '-' };
    let x = if bits & 0o1 != 0 { 'x' } else { '-' };
    format!("{}{}{}", r, w, x)
}

// ANSI color codes
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD_BLUE: &str = "\x1b[1;34m";
    pub const BOLD_GREEN: &str = "\x1b[1;32m";
    pub const GRAY: &str = "\x1b[90m";
    pub const RED: &str = "\x1b[31m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const CYAN: &str = "\x1b[36m";

    /// Wrap text with a color code
    pub fn wrap(text: &str, color: &str) -> String {
        format!("{}{}{}", color, text, RESET)
    }

    /// Colorize a filename based on metadata
    pub fn colorize_file(name: &str, metadata: Option<&std::fs::Metadata>) -> String {
        use std::os::unix::fs::PermissionsExt;

        match metadata {
            Some(m) => {
                if m.is_dir() {
                    wrap(name, BOLD_BLUE)
                } else if m.permissions().mode() & 0o111 != 0 {
                    wrap(name, BOLD_GREEN)
                } else if name.starts_with('.') {
                    wrap(name, GRAY)
                } else {
                    name.to_string()
                }
            }
            None => name.to_string(),
        }
    }
}
