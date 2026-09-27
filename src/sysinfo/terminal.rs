use colored::ColoredString;
use std::env;
use std::fmt::Write;

pub fn get_shell(buf: &mut String, c: fn(&str) -> ColoredString) {
    let shell_name = if let Ok(shell_path) = env::var("SHELL") {
        shell_path
            .split('/')
            .next_back()
            .unwrap_or("Unknown")
            .to_string()
    } else {
        env::var("ComSpec")
            .map(|p| p.split('\\').next_back().unwrap_or("cmd.exe").to_string())
            .unwrap_or_else(|_| "Unknown".to_string())
    };

    let _ = writeln!(buf, "{}: {}", c("Shell"), shell_name);
}

pub fn terminal_info(buf: &mut String, c: fn(&str) -> ColoredString) {
    let term = if cfg!(windows) {
        "cmd".to_string()
    } else if let Ok(term) = env::var("TERM_PROGRAM") {
        if !term.is_empty() {
            term
        } else {
            get_fallback_term()
        }
    } else {
        get_fallback_term()
    };

    let _ = writeln!(buf, "{}: {}", c("Terminal"), term);
}

fn get_fallback_term() -> String {
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
    if env::var("WT_SESSION").is_ok() {
        return "Windows Terminal".to_string();
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

    "Unknown".to_string()
}
