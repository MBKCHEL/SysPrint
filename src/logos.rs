use colored::*;
use std::env;
use std::path::Path;
use sysinfo::System;

macro_rules! include_logo {
    ($file:expr) => {
        include_str!(concat!("../assets/normal/", $file))
    };
}

pub struct LogoLine {
    pub text: String,
    pub width: usize,
}

fn flag_color(flag: &str) -> Option<Option<&'static str>> {
    match flag.to_ascii_lowercase().as_str() {
        "r" | "red" => Some(Some("red")),
        "gr" | "gray" => Some(Some("gray")),
        "g" | "green" => Some(Some("green")),
        "b" | "blue" => Some(Some("blue")),
        "y" | "yellow" => Some(Some("yellow")),
        "w" | "white" => Some(Some("white")),
        "k" | "black" => Some(Some("black")),
        "m" | "magenta" | "p" | "purple" => Some(Some("magenta")),
        "c" | "cyan" => Some(Some("cyan")),
        "o" | "orange" => Some(Some("orange")),
        "d" | "default" | "reset" => Some(None),
        _ => None,
    }
}

fn parse_logo_line(line: &str, default_color: &str, force_color: Option<&str>) -> LogoLine {
    let mut text = String::new();
    let mut width = 0usize;
    let mut current_color = force_color.unwrap_or(default_color);
    let mut segment = String::new();

    let flush = |segment: &mut String, color: &str, text: &mut String| {
        if !segment.is_empty() {
            text.push_str(&parse_color(segment, color).to_string());
            segment.clear();
        }
    };

    let mut i = 0usize;
    while i < line.len() {
        if line.as_bytes()[i] == b'{' {
            if let Some(rel_end) = line[i + 1..].find('}') {
                let end = i + 1 + rel_end;
                let flag = &line[i + 1..end];
                if let Some(color) = flag_color(flag) {
                    flush(&mut segment, current_color, &mut text);
                    if force_color.is_none() {
                        current_color = color.unwrap_or(default_color);
                    }
                    i = end + 1;
                    continue;
                }
            }
        }

        let ch = line[i..].chars().next().expect("valid char boundary");
        segment.push(ch);
        width += 1;
        i += ch.len_utf8();
    }

    flush(&mut segment, current_color, &mut text);

    LogoLine { text, width }
}

pub fn parse_color(text: &str, color_str: &str) -> ColoredString {
    let color_lower = color_str.trim().to_lowercase();

    if color_lower.starts_with('#') {
        let hex = color_lower.trim_start_matches('#');
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                return text.truecolor(r, g, b).bold();
            }
        }
    }

    match color_lower.as_str() {
        "red" => text.red().bold(),
        "green" => text.green().bold(),
        "blue" => text.blue().bold(),
        "yellow" => text.yellow().bold(),
        "magenta" | "purple" => text.magenta().bold(),
        "cyan" => text.cyan().bold(),
        "white" => text.white().bold(),
        "black" => text.black().bold(),
        "orange" => text.truecolor(255, 165, 0).bold(),
        _ => text.white().bold(),
    }
}

pub fn get_logo(
    logo: &str,
    custom_color: &str,
    custom_logo_path: &str,
) -> (Vec<LogoLine>, usize, String) {
    let mut os_name = System::name().unwrap_or_default().to_lowercase();
    let mut file_content = String::new();
    let mut is_custom = false;
    let mut final_path = custom_logo_path.to_string();

    // Android / Termux
    let is_android = Path::new("/system/build.prop").exists()
        || env::var("PREFIX")
            .map(|p| p.contains("com.termux"))
            .unwrap_or(false);

    if is_android {
        os_name = "android".to_string();
    }

    if final_path.starts_with('~') {
        if let Some(home_dir) = env::var_os("HOME") {
            let home_str = home_dir.to_string_lossy();
            final_path = final_path.replacen('~', &home_str, 1);
        }
    }

    if !custom_logo_path.is_empty() && custom_logo_path != "none" {
        match std::fs::read_to_string(Path::new(&final_path)) {
            Ok(content) => {
                file_content = content;
                is_custom = true;
            }
            Err(e) => {
                eprintln!("Error read custom logo '{}': {}", final_path, e);
            }
        }
    }


    if logo != "default" {
        os_name = logo.to_string().to_lowercase();
    }

    let (raw_logo, default_color) = if is_custom {
        (&file_content as &str, "white")
    } else {
        match os_name.as_str() {
            // Arch Based
            s if s.contains("cachyos") => (include_logo!("cachyos.txt"), "green"),
            s if s.contains("manjaro") => (include_logo!("manjaro.txt"), "green"),
            s if s.contains("endeavour") => (include_logo!("endeavour.txt"), "purple"),
            s if s.contains("artix") => (include_logo!("artix.txt"), "blue"),
            s if s.contains("arch") => (include_logo!("arch.txt"), "blue"),

            // Debian Based
            s if s.contains("kali") => (include_logo!("kali.txt"), "white"),
            s if s.contains("astra") => (include_logo!("astra.txt"), "blue"),
            s if s.contains("debian") => (include_logo!("debian.txt"), "red"),

            // Ubuntu Based
            s if s.contains("mint") => (include_logo!("mint.txt"), "green"),
            s if s.contains("zorinos") || s.contains("zorin") => {
                (include_logo!("zorin.txt"), "blue")
            }
            s if s.contains("pop") || s.contains("popos") => {
                (include_logo!("popos.txt"), "blue")
            }
            s if s.contains("lubuntu") => (include_logo!("lubuntu.txt"), "blue"),
            s if s.contains("kubuntu") => (include_logo!("kubuntu.txt"), "blue"),
            s if s.contains("xubuntu") => (include_logo!("xubuntu.txt"), "blue"),
            s if s.contains("ubuntu") => (include_logo!("ubuntu.txt"), "orange"),

            // BSD
            s if s.contains("freebsd") => (include_logo!("freebsd.txt"), "red"),
            s if s.contains("netbsd") => (include_logo!("netbsd.txt"), "orange"),
            s if s.contains("openbsd") => (include_logo!("openbsd.txt"), "orange"),

            // Independent
            s if s.contains("android") => (include_logo!("android.txt"), "green"),
            s if s.contains("opensuse") => (include_logo!("opensuse.txt"), "green"),
            s if s.contains("nixos") => (include_logo!("nixos.txt"), "blue"),
            s if s.contains("void") => (include_logo!("void.txt"), "green"),
            s if s.contains("windows") => (include_logo!("windows.txt"), "blue"),
            s if s.contains("fedora") => (include_logo!("fedora.txt"), "blue"),
            s if s.contains("gentoo") => (include_logo!("gentoo.txt"), "white"),
            s if s.contains("alpine") => (include_logo!("alpine.txt"), "purple"),
            s if s.contains("darwin") || s.contains("mac") => (include_logo!("apple.txt"), "white"),

            // Default (Tux)
            s if s.contains("tux") => (include_logo!("tux.txt"), "white"),
            _ => (include_logo!("tux.txt"), "white"),
        }
    };

    let active_color = if custom_color != "default" {
        custom_color
    } else {
        default_color
    };
    let force_color = (custom_color != "default").then_some(active_color);

    let logo_lines: Vec<LogoLine> = raw_logo
        .lines()
        .map(|line| parse_logo_line(line, active_color, force_color))
        .collect();

    let max_width = logo_lines.iter().map(|line| line.width).max().unwrap_or(0);

    (logo_lines, max_width, active_color.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_color_flags_and_counts_visible_width() {
        let line = parse_logo_line("{r}abc{g}de", "white", None);
        assert_eq!(line.width, 5);
        assert!(!line.text.contains("{r}"));
        assert!(!line.text.contains("{g}"));
    }

    #[test]
    fn unknown_flags_are_literal_text() {
        let line = parse_logo_line("{x}ab", "white", None);
        assert_eq!(line.width, 5);
        assert!(line.text.contains("{x}"));
    }

    #[test]
    fn forced_color_strips_flags() {
        let line = parse_logo_line("{r}abc", "white", Some("blue"));
        assert_eq!(line.width, 3);
        assert!(!line.text.contains("{r}"));
    }
}
