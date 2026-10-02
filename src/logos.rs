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

pub fn get_logo(mini_logo: bool) -> (Vec<ColoredString>, usize, fn(&str) -> ColoredString) {
    let mut os_name = System::name().unwrap_or_default().to_lowercase();

    // Android / Termux
    let is_android = Path::new("/system/build.prop").exists()
        || env::var("PREFIX")
            .map(|p| p.contains("com.termux"))
            .unwrap_or(false);

    if is_android {
        os_name = "android".to_string();
    }

    let (raw_logo, color_func): (&'static str, fn(&str) -> ColoredString) =
        match os_name.as_str() {
            // Arch Based
            s if s.contains("cachyos") => (include_logo!("cachyos.txt", mini_logo), |s| s.green().bold()),
            s if s.contains("manjaro") => (include_logo!("manjaro.txt", mini_logo), |s| s.green().bold()),
            s if s.contains("endeavour") => (include_logo!("endeavour.txt", mini_logo), |s| s.purple().bold()),
            s if s.contains("artix") => (include_logo!("artix.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("arch") => (include_logo!("arch.txt", mini_logo), |s| s.blue().bold()),

            // Debian Based
            s if s.contains("kali") => (include_logo!("kali.txt", mini_logo), |s| s.white().bold()),
            s if s.contains("astra") => (include_logo!("astra.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("debian") => (include_logo!("debian.txt", mini_logo), |s| s.red().bold()),

            // Ubuntu Based
            s if s.contains("mint") => (include_logo!("mint.txt", mini_logo), |s| s.green().bold()),
            s if s.contains("zorinos") || s.contains("zorin") => (include_logo!("zorin.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("pop") || s.contains("popos") => (include_logo!("popos.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("lubuntu") => (include_logo!("lubuntu.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("kubuntu") => (include_logo!("kubuntu.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("xubuntu") => (include_logo!("xubuntu.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("ubuntu") => (include_logo!("ubuntu.txt", mini_logo), |s| s.red().bold()),

            // BSD
            s if s.contains("freebsd") => (include_logo!("freebsd.txt", mini_logo), |s| s.red().bold()),
            s if s.contains("netbsd") => (include_logo!("netbsd.txt", mini_logo), |s| s.yellow().bold()),
            s if s.contains("openbsd") => (include_logo!("openbsd.txt", mini_logo), |s| s.yellow().bold()),

            // Independent
            s if s.contains("android") => (include_logo!("android.txt", mini_logo), |s| s.green().bold()),
            s if s.contains("opensuse") => (include_logo!("opensuse.txt", mini_logo), |s| s.green().bold()),
            s if s.contains("nixos") => (include_logo!("nixos.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("void") => (include_logo!("void.txt", mini_logo), |s| s.cyan().bold()),
            s if s.contains("windows") => (include_logo!("windows.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("fedora") => (include_logo!("fedora.txt", mini_logo), |s| s.blue().bold()),
            s if s.contains("gentoo") => (include_logo!("gentoo.txt", mini_logo), |s| s.white().bold()),
            s if s.contains("alpine") => (include_logo!("alpine.txt", mini_logo), |s| s.purple().bold()),
            s if s.contains("darwin") || s.contains("mac") => (include_logo!("apple.txt", mini_logo), |s| s.white().bold()),

            // Default
            _ => (include_logo!("tux.txt", mini_logo), |s| s.white()),
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
            color_func(&format!("{line}{padding}"))
        })
        .collect();

    (logo_lines, max_width, color_func)
}
