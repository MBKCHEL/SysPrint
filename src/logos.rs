use sysinfo::System;
use colored::*;

const ARCH_LOGO: &str = include_str!("../assets/arch.txt");
const UBUNTU_LOGO: &str = include_str!("../assets/ubuntu.txt");
const APPLE_LOGO: &str = include_str!("../assets/apple.txt");
const DEBIAN_LOGO: &str = include_str!("../assets/debian.txt");
const FEDORA_LOGO: &str = include_str!("../assets/fedora.txt");
const GENTOO_LOGO: &str = include_str!("../assets/gentoo.txt");
const KALI_LOGO: &str = include_str!("../assets/kali.txt");
const MANJARO_LOGO: &str = include_str!("../assets/manjaro.txt");
const MINT_LOGO: &str = include_str!("../assets/mint.txt");
const NIXOS_LOGO: &str = include_str!("../assets/nixos.txt");
const VOID_LOGO: &str = include_str!("../assets/void.txt");
const WINDOWS_LOGO: &str = include_str!("../assets/windows.txt");
const NET_BSD_LOGO: &str = include_str!("../assets/netbsd.txt");
const OPEN_BSD_LOGO: &str = include_str!("../assets/openbsd.txt");
const FREE_BSD_LOGO: &str = include_str!("../assets/freebsd.txt");
const ANDROID_LOGO: &str = include_str!("../assets/android.txt");
const CACHYOS_LOGO: &str = include_str!("../assets/cachyos.txt");
const DEFAULT_LOGO: &str = include_str!("../assets/tux.txt");

pub fn get_logo() -> (Vec<ColoredString>, usize) {
    let os_name = System::name().unwrap_or_default().to_lowercase();
    let (raw_logo, color_func, _): (&str, fn(&str) -> ColoredString, usize) = match os_name {
        s if s.contains("arch") => (ARCH_LOGO, |s| s.blue().bold(), 16),
        s if s.contains("cachyos") => (CACHYOS_LOGO, |s| s.green().bold(), 16),
        s if s.contains("manjaro") => (MANJARO_LOGO, |s| s.green().bold(), 16),
        s if s.contains("android") => (ANDROID_LOGO, |s| s.green().bold(), 16),
        s if s.contains("openbsd") => (OPEN_BSD_LOGO, |s| s.yellow().bold(), 16),
        s if s.contains("freebsd") => (FREE_BSD_LOGO, |s| s.red().bold(), 16),
        s if s.contains("netbsd") => (NET_BSD_LOGO, |s| s.yellow().bold(), 1),
        s if s.contains("mint") => (MINT_LOGO, |s| s.green().bold(), 16),
        s if s.contains("nixos") => (NIXOS_LOGO, |s| s.blue().bold(), 16),
        s if s.contains("void") => (VOID_LOGO, |s| s.cyan().bold(), 16),
        s if s.contains("windows") => (WINDOWS_LOGO, |s| s.blue().bold(), 16),
        s if s.contains("ubuntu") => (UBUNTU_LOGO, |s| s.red().bold(), 16),
        s if s.contains("fedora") => (FEDORA_LOGO, |s| s.blue().bold(), 16),
        s if s.contains("gentoo") => (GENTOO_LOGO, |s| s.white().bold(), 16),
        s if s.contains("debian") => (DEBIAN_LOGO, |s| s.red().bold(), 16),
        s if s.contains("darwin") || s.contains("mac") || s.contains("macos") => {
            (APPLE_LOGO, |s| s.white().bold(), 16)
        }
        s if s.contains("kali") => (KALI_LOGO, |s| s.white().bold(), 16),
        _ => (DEFAULT_LOGO, |s| s.white(), 10),
    };

    let max_width = raw_logo.lines().map(|l| l.len()).max().unwrap_or(0);


    let logo_lines: Vec<ColoredString> = raw_logo
        .lines()
        .map(|line| {
            let padded = format!("{:width$}", line, width = max_width);
            color_func(&padded)
        })
        .collect();


    (logo_lines, max_width)
}