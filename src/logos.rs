use sysinfo::System;
use colored::*;

pub fn get_logo() -> (Vec<ColoredString>, usize) {
    let os_name = System::name().unwrap_or_default().to_lowercase();

    let (raw_logo, color_func): (&str, fn(&str) -> ColoredString) = match os_name.as_str() {
        s if s.contains("arch")    => (include_str!("../assets/arch.txt"), |s| s.blue().bold()),
        s if s.contains("cachyos")  => (include_str!("../assets/cachyos.txt"), |s| s.green().bold()),
        s if s.contains("manjaro")  => (include_str!("../assets/manjaro.txt"), |s| s.green().bold()),
        s if s.contains("android")  => (include_str!("../assets/android.txt"), |s| s.green().bold()),
        s if s.contains("openbsd")  => (include_str!("../assets/openbsd.txt"), |s| s.yellow().bold()),
        s if s.contains("freebsd")  => (include_str!("../assets/freebsd.txt"), |s| s.red().bold()),
        s if s.contains("netbsd")   => (include_str!("../assets/netbsd.txt"), |s| s.yellow().bold()),
        s if s.contains("mint")     => (include_str!("../assets/mint.txt"), |s| s.green().bold()),
        s if s.contains("nixos")    => (include_str!("../assets/nixos.txt"), |s| s.blue().bold()),
        s if s.contains("void")     => (include_str!("../assets/void.txt"), |s| s.cyan().bold()),
        s if s.contains("windows")  => (include_str!("../assets/windows.txt"), |s| s.blue().bold()),
        s if s.contains("ubuntu")   => (include_str!("../assets/ubuntu.txt"), |s| s.red().bold()),
        s if s.contains("fedora")   => (include_str!("../assets/fedora.txt"), |s| s.blue().bold()),
        s if s.contains("gentoo")   => (include_str!("../assets/gentoo.txt"), |s| s.white().bold()),
        s if s.contains("debian")   => (include_str!("../assets/debian.txt"), |s| s.red().bold()),
        s if s.contains("kali")     => (include_str!("../assets/kali.txt"), |s| s.white().bold()),
        s if s.contains("darwin") || s.contains("mac") => (include_str!("../assets/apple.txt"), |s| s.white().bold()),
        _ => (include_str!("../assets/tux.txt"), |s| s.white()),
    };

    
    let max_width = raw_logo.lines().map(|l| l.chars().count()).max().unwrap_or(0);

    let logo_lines = raw_logo
        .lines()
        .map(|line| {
            let char_count = line.chars().count();
            let padding = " ".repeat(max_width.saturating_sub(char_count));
            color_func(&format!("{line}{padding}"))
        })
        .collect();

    (logo_lines, max_width)
}