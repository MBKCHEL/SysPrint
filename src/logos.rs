use sysinfo::System;
use colored::*;
use std::path::Path;
use std::env;

pub fn is_valid_logo(name: &str) -> bool {
    let trimmed = name.trim();
    if Path::new(trimmed).is_file() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "alpine"
            | "almalinux"
            | "alt"
            | "altlinux"
            | "android"
            | "apple"
            | "arch"
            | "archlinux"
            | "artix"
            | "asahi"
            | "astra"
            | "cachyos"
            | "centos"
            | "chrome"
            | "chromeos"
            | "chromium"
            | "debian"
            | "deepin"
            | "elementary"
            | "endeavour"
            | "fedora"
            | "freebsd"
            | "garuda"
            | "gentoo"
            | "kali"
            | "kdeneon"
            | "kubuntu"
            | "lubuntu"
            | "manjaro"
            | "mint"
            | "mx"
            | "neon"
            | "netbsd"
            | "nixos"
            | "openbsd"
            | "opensuse"
            | "parrot"
            | "pop"
            | "popos"
            | "proxmox"
            | "raspbian"
            | "raspberry"
            | "redhat"
            | "rhel"
            | "rocky"
            | "slackware"
            | "solus"
            | "steamos"
            | "steamdeck"
            | "tails"
            | "truenas"
            | "tux"
            | "ubuntu"
            | "void"
            | "windows"
            | "xubuntu"
            | "zorin"
            | "zorinos"
            | "mac"
            | "darwin"
    )
}

pub fn get_logo(mini_logo: bool, custom_logo: Option<&str>) -> (Vec<ColoredString>, usize, fn(&str) -> ColoredString) {
    if let Some(custom) = custom_logo {
        let p = Path::new(custom.trim());
        if p.is_file()
            && let Ok(file_content) = std::fs::read_to_string(p)
        {
            let default_color: fn(&str) -> ColoredString = |s| s.white().bold();
            let max_width = file_content.lines().map(|l| l.chars().count()).max().unwrap_or(0);
            let logo_lines = file_content
                .lines()
                .map(|line| {
                    let char_count = line.chars().count();
                    let padding = " ".repeat(max_width.saturating_sub(char_count));
                    default_color(&format!("{line}{padding}"))
                })
                .collect();
            return (logo_lines, max_width, default_color);
        }
    }

    let os_name = match custom_logo {
        Some(name) if !name.trim().is_empty() => name.trim().to_lowercase(),
        _ => {
            let mut name = System::name().unwrap_or_default().to_lowercase();
            // Android/Termux
            let is_android = Path::new("/system/build.prop").exists()
                || env::var("PREFIX").map(|p| p.contains("com.termux")).unwrap_or(false);

            if is_android {
                name = "android".to_string();
            }
            name
        }
    };


    let (raw_logo, color_func): (&str, fn(&str) -> ColoredString) = if mini_logo {
        // Mini
        match os_name.as_str() {
            // Arch Based
            s if s.contains("cachyos")  => (include_str!("../assets/mini/c/cachyos.txt"), |s| s.green().bold()),
            s if s.contains("manjaro")  => (include_str!("../assets/mini/m/manjaro.txt"), |s| s.green().bold()),
            s if s.contains("endeavour") => (include_str!("../assets/mini/e/endeavour.txt"), |s| s.purple().bold()),
            s if s.contains("artix")    => (include_str!("../assets/mini/a/artix.txt"), |s| s.blue().bold()),
            s if s.contains("garuda")   => (include_str!("../assets/mini/g/garuda.txt"), |s| s.red().bold()),
            s if s.contains("asahi")    => (include_str!("../assets/mini/a/asahi.txt"), |s| s.red().bold()),
            s if s.contains("arch")    => (include_str!("../assets/mini/a/arch.txt"), |s| s.blue().bold()),

            // Debian Based
            s if s.contains("kali")     => (include_str!("../assets/mini/k/kali.txt"), |s| s.white().bold()),
            s if s.contains("parrot")   => (include_str!("../assets/mini/p/parrot.txt"), |s| s.cyan().bold()),
            s if s.contains("astra")    => (include_str!("../assets/mini/a/astra.txt"), |s| s.blue().bold()),
            s if s.contains("proxmox")  => (include_str!("../assets/mini/p/proxmox.txt"), |s| s.red().bold()),
            s if s.contains("tails")    => (include_str!("../assets/mini/t/tails.txt"), |s| s.purple().bold()),
            s if s.contains("deepin")   => (include_str!("../assets/mini/d/deepin.txt"), |s| s.cyan().bold()),
            s if s.contains("raspbian") || s.contains("raspberry") => (include_str!("../assets/mini/r/raspbian.txt"), |s| s.red().bold()),
            s if s.contains("debian")   => (include_str!("../assets/mini/d/debian.txt"), |s| s.red().bold()),

            // Ubuntu Based
            s if s.contains("mint")     => (include_str!("../assets/mini/m/mint.txt"), |s| s.green().bold()),
            s if s.contains("neon")     => (include_str!("../assets/mini/k/kdeneon.txt"), |s| s.cyan().bold()),
            s if s.contains("elementary") => (include_str!("../assets/mini/e/elementary.txt"), |s| s.cyan().bold()),
            s if s.contains("zorinos") || s.contains("zorin") => (include_str!("../assets/mini/z/zorin.txt"), |s| s.blue().bold()),
            s if s.contains("pop") || s.contains("popos")     => (include_str!("../assets/mini/p/popos.txt"), |s| s.blue().bold()),
            s if s.contains("lubuntu") => (include_str!("../assets/mini/l/lubuntu.txt"), |s| s.blue().bold()),
            s if s.contains("kubuntu") => (include_str!("../assets/mini/k/kubuntu.txt"), |s| s.blue().bold()),
            s if s.contains("xubuntu") => (include_str!("../assets/mini/x/xubuntu.txt"), |s| s.blue().bold()),
            s if s.contains("ubuntu")   => (include_str!("../assets/mini/u/ubuntu.txt"), |s| s.red().bold()),

            // RHEL / Enterprise Based
            s if s.contains("centos")   => (include_str!("../assets/mini/c/centos.txt"), |s| s.yellow().bold()),
            s if s.contains("rhel") || s.contains("redhat") => (include_str!("../assets/mini/r/rhel.txt"), |s| s.red().bold()),
            s if s.contains("rocky")    => (include_str!("../assets/mini/r/rocky.txt"), |s| s.green().bold()),
            s if s.contains("alma")     => (include_str!("../assets/mini/a/almalinux.txt"), |s| s.blue().bold()),

            // BSD & Storage
            s if s.contains("truenas")  => (include_str!("../assets/mini/t/truenas.txt"), |s| s.cyan().bold()),
            s if s.contains("freebsd")  => (include_str!("../assets/mini/f/freebsd.txt"), |s| s.red().bold()),
            s if s.contains("netbsd")   => (include_str!("../assets/mini/n/netbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("openbsd")  => (include_str!("../assets/mini/o/openbsd.txt"), |s| s.yellow().bold()),

            // Independent & Others
            s if s.contains("steamos") || s.contains("steamdeck") => (include_str!("../assets/mini/s/steamos.txt"), |s| s.blue().bold()),
            s if s.contains("chrome")   => (include_str!("../assets/mini/c/chromeos.txt"), |s| s.green().bold()),
            s if s.contains("mx")       => (include_str!("../assets/mini/m/mx.txt"), |s| s.white().bold()),
            s if s.contains("slackware") => (include_str!("../assets/mini/s/slackware.txt"), |s| s.blue().bold()),
            s if s.contains("solus")    => (include_str!("../assets/mini/s/solus.txt"), |s| s.blue().bold()),
            s if s.contains("alt")      => (include_str!("../assets/mini/a/altlinux.txt"), |s| s.yellow().bold()),
            s if s.contains("android")  => (include_str!("../assets/mini/a/android.txt"), |s| s.green().bold()),
            s if s.contains("opensuse") => (include_str!("../assets/mini/o/opensuse.txt"), |s| s.green().bold()),
            s if s.contains("nixos")    => (include_str!("../assets/mini/n/nixos.txt"), |s| s.blue().bold()),
            s if s.contains("void")     => (include_str!("../assets/mini/v/void.txt"), |s| s.cyan().bold()),
            s if s.contains("windows")  => (include_str!("../assets/mini/w/windows.txt"), |s| s.blue().bold()),
            s if s.contains("fedora")   => (include_str!("../assets/mini/f/fedora.txt"), |s| s.blue().bold()),
            s if s.contains("gentoo")   => (include_str!("../assets/mini/g/gentoo.txt"), |s| s.white().bold()),
            s if s.contains("alpine")   => (include_str!("../assets/mini/a/alpine.txt"), |s| s.purple().bold()),
            s if s.contains("darwin") || s.contains("mac")    => (include_str!("../assets/mini/a/apple.txt"), |s| s.white().bold()),
            _ => (include_str!("../assets/mini/t/tux.txt"), |s| s.white()),
        }
    } else {
        // Normal
        match os_name.as_str() {
            // Arch Based
            s if s.contains("cachyos")  => (include_str!("../assets/normal/c/cachyos.txt"), |s| s.green().bold()),
            s if s.contains("manjaro")  => (include_str!("../assets/normal/m/manjaro.txt"), |s| s.green().bold()),
            s if s.contains("endeavour") => (include_str!("../assets/normal/e/endeavour.txt"), |s| s.purple().bold()),
            s if s.contains("artix")    => (include_str!("../assets/normal/a/artix.txt"), |s| s.blue().bold()),
            s if s.contains("garuda")   => (include_str!("../assets/normal/g/garuda.txt"), |s| s.red().bold()),
            s if s.contains("asahi")    => (include_str!("../assets/normal/a/asahi.txt"), |s| s.red().bold()),
            s if s.contains("arch")    => (include_str!("../assets/normal/a/arch.txt"), |s| s.blue().bold()),

            // Debian Based
            s if s.contains("kali")     => (include_str!("../assets/normal/k/kali.txt"), |s| s.white().bold()),
            s if s.contains("parrot")   => (include_str!("../assets/normal/p/parrot.txt"), |s| s.cyan().bold()),
            s if s.contains("astra")    => (include_str!("../assets/normal/a/astra.txt"), |s| s.blue().bold()),
            s if s.contains("proxmox")  => (include_str!("../assets/normal/p/proxmox.txt"), |s| s.red().bold()),
            s if s.contains("tails")    => (include_str!("../assets/normal/t/tails.txt"), |s| s.purple().bold()),
            s if s.contains("deepin")   => (include_str!("../assets/normal/d/deepin.txt"), |s| s.cyan().bold()),
            s if s.contains("raspbian") || s.contains("raspberry") => (include_str!("../assets/normal/r/raspbian.txt"), |s| s.red().bold()),
            s if s.contains("debian")   => (include_str!("../assets/normal/d/debian.txt"), |s| s.red().bold()),

            // Ubuntu Based
            s if s.contains("mint")     => (include_str!("../assets/normal/m/mint.txt"), |s| s.green().bold()),
            s if s.contains("neon")     => (include_str!("../assets/normal/k/kdeneon.txt"), |s| s.cyan().bold()),
            s if s.contains("elementary") => (include_str!("../assets/normal/e/elementary.txt"), |s| s.cyan().bold()),
            s if s.contains("zorinos") || s.contains("zorin") => (include_str!("../assets/normal/z/zorin.txt"), |s| s.blue().bold()),
            s if s.contains("pop") || s.contains("popos")     => (include_str!("../assets/normal/p/popos.txt"), |s| s.blue().bold()),
            s if s.contains("lubuntu") => (include_str!("../assets/normal/l/lubuntu.txt"), |s| s.blue().bold()),
            s if s.contains("kubuntu") => (include_str!("../assets/normal/k/kubuntu.txt"), |s| s.blue().bold()),
            s if s.contains("xubuntu") => (include_str!("../assets/normal/x/xubuntu.txt"), |s| s.blue().bold()),
            s if s.contains("ubuntu")   => (include_str!("../assets/normal/u/ubuntu.txt"), |s| s.red().bold()),

            // RHEL / Enterprise Based
            s if s.contains("centos")   => (include_str!("../assets/normal/c/centos.txt"), |s| s.yellow().bold()),
            s if s.contains("rhel") || s.contains("redhat") => (include_str!("../assets/normal/r/rhel.txt"), |s| s.red().bold()),
            s if s.contains("rocky")    => (include_str!("../assets/normal/r/rocky.txt"), |s| s.green().bold()),
            s if s.contains("alma")     => (include_str!("../assets/normal/a/almalinux.txt"), |s| s.blue().bold()),

            // BSD & Storage
            s if s.contains("truenas")  => (include_str!("../assets/normal/t/truenas.txt"), |s| s.cyan().bold()),
            s if s.contains("freebsd")  => (include_str!("../assets/normal/f/freebsd.txt"), |s| s.red().bold()),
            s if s.contains("netbsd")   => (include_str!("../assets/normal/n/netbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("openbsd")  => (include_str!("../assets/normal/o/openbsd.txt"), |s| s.yellow().bold()),

            // Independent & Others
            s if s.contains("steamos") || s.contains("steamdeck") => (include_str!("../assets/normal/s/steamos.txt"), |s| s.blue().bold()),
            s if s.contains("chrome")   => (include_str!("../assets/normal/c/chromeos.txt"), |s| s.green().bold()),
            s if s.contains("mx")       => (include_str!("../assets/normal/m/mx.txt"), |s| s.white().bold()),
            s if s.contains("slackware") => (include_str!("../assets/normal/s/slackware.txt"), |s| s.blue().bold()),
            s if s.contains("solus")    => (include_str!("../assets/normal/s/solus.txt"), |s| s.blue().bold()),
            s if s.contains("alt")      => (include_str!("../assets/normal/a/altlinux.txt"), |s| s.yellow().bold()),
            s if s.contains("android")  => (include_str!("../assets/normal/a/android.txt"), |s| s.green().bold()),
            s if s.contains("opensuse") => (include_str!("../assets/normal/o/opensuse.txt"), |s| s.green().bold()),
            s if s.contains("nixos")    => (include_str!("../assets/normal/n/nixos.txt"), |s| s.blue().bold()),
            s if s.contains("void")     => (include_str!("../assets/normal/v/void.txt"), |s| s.cyan().bold()),
            s if s.contains("windows")  => (include_str!("../assets/normal/w/windows.txt"), |s| s.blue().bold()),
            s if s.contains("fedora")   => (include_str!("../assets/normal/f/fedora.txt"), |s| s.blue().bold()),
            s if s.contains("gentoo")   => (include_str!("../assets/normal/g/gentoo.txt"), |s| s.white().bold()),
            s if s.contains("alpine")   => (include_str!("../assets/normal/a/alpine.txt"), |s| s.purple().bold()),
            s if s.contains("darwin") || s.contains("mac")    => (include_str!("../assets/normal/a/apple.txt"), |s| s.white().bold()),
            _ => (include_str!("../assets/normal/t/tux.txt"), |s| s.white()),
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

    (logo_lines, max_width, color_func)
}

pub fn resolve_color(color_name: &str) -> Option<fn(&str) -> ColoredString> {
    match color_name.trim().to_lowercase().as_str() {
        "cyan" | "bright-cyan" | "bright_cyan" => Some(|s| s.bright_cyan().bold()),
        "blue" | "bright-blue" | "bright_blue" => Some(|s| s.bright_blue().bold()),
        "green" | "bright-green" | "bright_green" => Some(|s| s.bright_green().bold()),
        "red" | "bright-red" | "bright_red" => Some(|s| s.bright_red().bold()),
        "yellow" | "bright-yellow" | "bright_yellow" => Some(|s| s.bright_yellow().bold()),
        "magenta" | "purple" | "bright-magenta" | "bright_magenta" => Some(|s| s.bright_magenta().bold()),
        "white" | "bright-white" | "bright_white" => Some(|s| s.bright_white().bold()),
        "black" => Some(|s| s.black().bold()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_logo() {
        assert!(is_valid_logo("arch"));
        assert!(is_valid_logo("Debian"));
        assert!(is_valid_logo(" Ubuntu "));
        assert!(is_valid_logo("windows"));
        assert!(is_valid_logo("mac"));
        assert!(is_valid_logo("tux"));
        assert!(is_valid_logo("archlinux"));
        assert!(!is_valid_logo("unknown_os"));
    }

    #[test]
    fn test_custom_logo_override() {
        let (lines, width, _) = get_logo(false, Some("debian"));
        assert!(!lines.is_empty());
        assert!(width > 0);
    }

    #[test]
    fn test_resolve_color() {
        assert!(resolve_color("cyan").is_some());
        assert!(resolve_color("red").is_some());
        assert!(resolve_color("magenta").is_some());
        assert!(resolve_color("invalid_color").is_none());
    }

    #[test]
    fn test_file_logo() {
        let tmp = std::env::temp_dir().join("test_logo.txt");
        std::fs::write(&tmp, "HELLO\nWORLD").unwrap();
        assert!(is_valid_logo(tmp.to_str().unwrap()));
        let (lines, width, _) = get_logo(false, Some(tmp.to_str().unwrap()));
        assert_eq!(lines.len(), 2);
        assert_eq!(width, 5);
        let _ = std::fs::remove_file(tmp);
    }
}