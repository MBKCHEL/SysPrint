use colored::ColoredString;
use std::env;
use std::fmt::Write;

pub fn get_shell(fast_mode: bool, buf: &mut String, c: fn(&str) -> ColoredString) {
    let display = get_shell_string(fast_mode);
    let _ = writeln!(buf, "{}: {}", c("Shell"), display);
}

pub fn get_shell_string(fast_mode: bool) -> String {
    let (shell_name, shell_path) = if let Ok(path) = env::var("SHELL") {
        let name = path
            .split('/')
            .next_back()
            .unwrap_or("Unknown")
            .to_string();
        (name, Some(path))
    } else {
        let comspec = env::var("ComSpec").unwrap_or_else(|_| "cmd.exe".to_string());
        let name = comspec
            .split('\\')
            .next_back()
            .unwrap_or("cmd.exe")
            .to_string();
        (name, Some(comspec))
    };

    let version = if fast_mode {
        None
    } else {
        get_shell_version(&shell_name, shell_path.as_deref())
    };

    match version {
        Some(ver) => format!("{shell_name} {ver}"),
        None => shell_name,
    }
}

fn get_shell_version(name: &str, path: Option<&str>) -> Option<String> {
    if name == "bash"
        && let Ok(v) = env::var("BASH_VERSION")
        && let Some(ver) = extract_version(&v)
    {
        return Some(ver);
    } else if name == "zsh"
        && let Ok(v) = env::var("ZSH_VERSION")
        && let Some(ver) = extract_version(&v)
    {
        return Some(ver);
    }

    let bin = path.unwrap_or(name);
    let output = std::process::Command::new(bin)
        .arg("--version")
        .output()
        .ok()?;

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        extract_version(&text)
    } else {
        None
    }
}

fn extract_version(text: &str) -> Option<String> {
    for line in text.lines() {
        for word in line.split_whitespace() {
            let clean = word
                .split('(')
                .next()
                .unwrap_or("")
                .trim_matches(|c: char| !c.is_ascii_digit());
            let parts: Vec<&str> = clean.split('.').collect();
            if parts.len() >= 2
                && parts.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
            {
                return Some(clean.to_string());
            }
        }
    }
    None
}

pub fn get_terminal() -> String {
    if let Ok(term) = env::var("TERM_PROGRAM")
        && !term.is_empty()
    {
        return term;
    }
    if env::var("WT_SESSION").is_ok() {
        return "Windows Terminal".to_string();
    }
    if env::var("KITTY_WINDOW_ID").is_ok() {
        return "kitty".to_string();
    }
    if env::var("ALACRITTY_SOCKET").is_ok() || env::var("ALACRITTY_LOG").is_ok() {
        return "alacritty".to_string();
    }
    if env::var("KONSOLE_VERSION").is_ok() {
        return "konsole".to_string();
    }
    if env::var("FOOT_SOCKET").is_ok() {
        return "foot".to_string();
    }
    if let Ok(term) = env::var("TERMINAL")
        && !term.is_empty()
    {
        return term;
    }
    if let Ok(term) = env::var("TERM")
        && term != "xterm-256color"
        && !term.is_empty()
    {
        return term;
    }

    if cfg!(windows) {
        "cmd".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn terminal_info(buf: &mut String, c: fn(&str) -> ColoredString) {
    let term = get_terminal();
    let _ = writeln!(buf, "{}: {}", c("Terminal"), term);
}
