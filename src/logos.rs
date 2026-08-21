use std::net::ToSocketAddrs;
use sysinfo::System;
use colored::*;

pub fn get_logo(mini: bool) -> (Vec<ColoredString>, usize) {
    let os_name = System::name().unwrap_or_default().to_lowercase();

    
    let (raw_logo, color_func): (&str, fn(&str) -> ColoredString) = if mini {
        // Mini
        match os_name.as_str() {
            s if s.contains("arch")    => (include_str!("../assets/mini/arch.txt"), |s| s.blue().bold()),
            s if s.contains("cachyos")  => (include_str!("../assets/mini/cachyos.txt"), |s| s.green().bold()),
            s if s.contains("manjaro")  => (include_str!("../assets/mini/manjaro.txt"), |s| s.green().bold()),
            s if s.contains("android")  => (include_str!("../assets/mini/android.txt"), |s| s.green().bold()),
            s if s.contains("openbsd")  => (include_str!("../assets/mini/openbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("freebsd")  => (include_str!("../assets/mini/freebsd.txt"), |s| s.red().bold()),
            s if s.contains("netbsd")   => (include_str!("../assets/mini/netbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("mint")     => (include_str!("../assets/mini/mint.txt"), |s| s.green().bold()),
            s if s.contains("nixos")    => (include_str!("../assets/mini/nixos.txt"), |s| s.blue().bold()),
            s if s.contains("void")     => (include_str!("../assets/mini/void.txt"), |s| s.cyan().bold()),
            s if s.contains("windows")  => (include_str!("../assets/mini/windows.txt"), |s| s.blue().bold()),
            s if s.contains("ubuntu")   => (include_str!("../assets/mini/ubuntu.txt"), |s| s.red().bold()),
            s if s.contains("fedora")   => (include_str!("../assets/mini/fedora.txt"), |s| s.blue().bold()),
            s if s.contains("gentoo")   => (include_str!("../assets/mini/gentoo.txt"), |s| s.white().bold()),
            s if s.contains("debian")   => (include_str!("../assets/mini/debian.txt"), |s| s.red().bold()),
            s if s.contains("kali")     => (include_str!("../assets/mini/kali.txt"), |s| s.white().bold()),
            s if s.contains("artix")    => (include_str!("../assets/mini/artix.txt"), |s| s.blue().bold()),
            s if s.contains("astra")    => (include_str!("../assets/mini/astra.txt"), |s| s.blue().bold()),
            s if s.contains("alpine")    => (include_str!("../assets/mini/alpine.txt"), |s| s.purple().bold()),
            s if s.contains("zorinos") || s.contains("zorin") => (include_str!("../assets/mini/zorin.txt"), |s| s.blue().bold()),
            s if s.contains("pop") || s.contains("popos")     => (include_str!("../assets/mini/popos.txt"), |s| s.blue().bold()),
            s if s.contains("darwin") || s.contains("mac")    => (include_str!("../assets/mini/apple.txt"), |s| s.white().bold()),
            _ => (include_str!("../assets/mini/tux.txt"), |s| s.white()),
        }
    } else {
        // Normal
        match os_name.as_str() {
            s if s.contains("arch")    => (include_str!("../assets/normal/arch.txt"), |s| s.blue().bold()),
            s if s.contains("cachyos")  => (include_str!("../assets/normal/cachyos.txt"), |s| s.green().bold()),
            s if s.contains("manjaro")  => (include_str!("../assets/normal/manjaro.txt"), |s| s.green().bold()),
            s if s.contains("android")  => (include_str!("../assets/normal/android.txt"), |s| s.green().bold()),
            s if s.contains("openbsd")  => (include_str!("../assets/normal/openbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("freebsd")  => (include_str!("../assets/normal/freebsd.txt"), |s| s.red().bold()),
            s if s.contains("netbsd")   => (include_str!("../assets/normal/netbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("mint")     => (include_str!("../assets/normal/mint.txt"), |s| s.green().bold()),
            s if s.contains("nixos")    => (include_str!("../assets/normal/nixos.txt"), |s| s.blue().bold()),
            s if s.contains("void")     => (include_str!("../assets/normal/void.txt"), |s| s.cyan().bold()),
            s if s.contains("windows")  => (include_str!("../assets/normal/windows.txt"), |s| s.blue().bold()),
            s if s.contains("ubuntu")   => (include_str!("../assets/normal/ubuntu.txt"), |s| s.red().bold()),
            s if s.contains("fedora")   => (include_str!("../assets/normal/fedora.txt"), |s| s.blue().bold()),
            s if s.contains("gentoo")   => (include_str!("../assets/normal/gentoo.txt"), |s| s.white().bold()),
            s if s.contains("debian")   => (include_str!("../assets/normal/debian.txt"), |s| s.red().bold()),
            s if s.contains("kali")     => (include_str!("../assets/normal/kali.txt"), |s| s.white().bold()),
            s if s.contains("artix")    => (include_str!("../assets/normal/artix.txt"), |s| s.blue().bold()),
            s if s.contains("astra")    => (include_str!("../assets/normal/astra.txt"), |s| s.blue().bold()),
            s if s.contains("alpine")    => (include_str!("../assets/normal/alpine.txt"), |s| s.purple().bold()),
            s if s.contains("zorinos") || s.contains("zorin") => (include_str!("../assets/normal/zorin.txt"), |s| s.blue().bold()),
            s if s.contains("pop") || s.contains("popos")     => (include_str!("../assets/normal/popos.txt"), |s| s.blue().bold()),
            s if s.contains("darwin") || s.contains("mac")    => (include_str!("../assets/normal/apple.txt"), |s| s.white().bold()),
            _ => (include_str!("../assets/normal/tux.txt"), |s| s.white()),
        }
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