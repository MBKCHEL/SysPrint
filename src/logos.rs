use colored::*;
use std::env;
use std::path::Path;
use sysinfo::System;

macro_rules! include_logo {
    ($file:expr, $is_mini:expr) => {
        if $is_mini {
            include_str!(concat!("../assets/mini/", $file))
        } else {
            include_str!(concat!("../assets/normal/", $file))
        }
    };
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
    mini_logo: bool,
    logo: &str,
    custom_color: &str,
    custom_logo_path: &str,
) -> (Vec<ColoredString>, usize, String) {
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
        if let Some(home_dir) = std::env::var_os("HOME") {
            let home_str = home_dir.to_string_lossy();
            final_path = final_path.replacen('~', &home_str, 1);
        }
    }

    if let Ok(content) = std::fs::read_to_string(std::path::Path::new(&final_path)) {
        file_content = content;
        is_custom = true;
    }


    if logo != "default" {
        os_name = logo.to_string().to_lowercase();
    }

    let (raw_logo, default_color) = if is_custom {
        (&file_content as &str, "white")
    } else {
        match os_name.as_str() {
            // Arch Based
            s if s.contains("cachyos") => (include_logo!("cachyos.txt", mini_logo), "green"),
            s if s.contains("manjaro") => (include_logo!("manjaro.txt", mini_logo), "green"),
            s if s.contains("endeavour") => (include_logo!("endeavour.txt", mini_logo), "purple"),
            s if s.contains("artix") => (include_logo!("artix.txt", mini_logo), "blue"),
            s if s.contains("arch") => (include_logo!("arch.txt", mini_logo), "blue"),

            // Debian Based
            s if s.contains("kali") => (include_logo!("kali.txt", mini_logo), "white"),
            s if s.contains("astra") => (include_logo!("astra.txt", mini_logo), "blue"),
            s if s.contains("debian") => (include_logo!("debian.txt", mini_logo), "red"),

            // Ubuntu Based
            s if s.contains("mint") => (include_logo!("mint.txt", mini_logo), "green"),
            s if s.contains("zorinos") || s.contains("zorin") => {
                (include_logo!("zorin.txt", mini_logo), "blue")
            }
            s if s.contains("pop") || s.contains("popos") => {
                (include_logo!("popos.txt", mini_logo), "blue")
            }
            s if s.contains("lubuntu") => (include_logo!("lubuntu.txt", mini_logo), "blue"),
            s if s.contains("kubuntu") => (include_logo!("kubuntu.txt", mini_logo), "blue"),
            s if s.contains("xubuntu") => (include_logo!("xubuntu.txt", mini_logo), "blue"),
            s if s.contains("ubuntu") => (include_logo!("ubuntu.txt", mini_logo), "orange"),

            // BSD
            s if s.contains("freebsd") => (include_logo!("freebsd.txt", mini_logo), "red"),
            s if s.contains("netbsd") => (include_logo!("netbsd.txt", mini_logo), "orange"),
            s if s.contains("openbsd") => (include_logo!("openbsd.txt", mini_logo), "orange"),

            // Independent
            s if s.contains("android") => (include_logo!("android.txt", mini_logo), "green"),
            s if s.contains("opensuse") => (include_logo!("opensuse.txt", mini_logo), "green"),
            s if s.contains("nixos") => (include_logo!("nixos.txt", mini_logo), "blue"),
            s if s.contains("void") => (include_logo!("void.txt", mini_logo), "green"),
            s if s.contains("windows") => (include_logo!("windows.txt", mini_logo), "blue"),
            s if s.contains("fedora") => (include_logo!("fedora.txt", mini_logo), "blue"),
            s if s.contains("gentoo") => (include_logo!("gentoo.txt", mini_logo), "white"),
            s if s.contains("alpine") => (include_logo!("alpine.txt", mini_logo), "purple"),
            s if s.contains("darwin") || s.contains("mac") => (include_logo!("apple.txt", mini_logo), "white"),

            // Default (Tux)
            s if s.contains("tux") => (include_logo!("tux.txt", mini_logo), "white"),
            _ => (include_logo!("tux.txt", mini_logo), "white"),
        }
    };

    let active_color = if custom_color != "default" {
        custom_color
    } else {
        default_color
    };

    let max_width = raw_logo
        .lines()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);

    let logo_lines = raw_logo
        .lines()
        .map(|line| {
            let char_count = line.chars().count();
            let padding = " ".repeat(max_width.saturating_sub(char_count));
            parse_color(&format!("{line}{padding}"), active_color)
        })
        .collect();

    (logo_lines, max_width, active_color.to_string())
}
