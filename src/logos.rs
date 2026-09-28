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
            | "amazon"
            | "amazonlinux"
            | "amzn"
            | "android"
            | "antix"
            | "apple"
            | "arch"
            | "archlinux"
            | "artix"
            | "asahi"
            | "astra"
            | "athena"
            | "athenaos"
            | "azure"
            | "azurelinux"
            | "bazzite"
            | "bedrock"
            | "bodhi"
            | "bodhilinux"
            | "bsd"
            | "cachyos"
            | "centos"
            | "chimera"
            | "chimeralinux"
            | "chrome"
            | "chromeos"
            | "chromium"
            | "clear"
            | "clearlinux"
            | "cosmic"
            | "debian"
            | "deepin"
            | "devuan"
            | "dragonfly"
            | "dragonflybsd"
            | "elementary"
            | "endeavour"
            | "endeavouros"
            | "fedora"
            | "femboy"
            | "femboyos"
            | "freebsd"
            | "garuda"
            | "gentoo"
            | "ghostbsd"
            | "gnome"
            | "gnu"
            | "graphene"
            | "grapheneos"
            | "guix"
            | "kali"
            | "kaos"
            | "kde"
            | "kdelinux"
            | "kdeneon"
            | "kernel"
            | "kernelos"
            | "kubuntu"
            | "lubuntu"
            | "mageia"
            | "manjaro"
            | "midnightbsd"
            | "minix"
            | "mint"
            | "mx"
            | "neon"
            | "netbsd"
            | "nixos"
            | "nobara"
            | "nomadbsd"
            | "obsidian"
            | "obsidianos"
            | "omarchy"
            | "openbsd"
            | "opensuse"
            | "openwrt"
            | "oracle"
            | "oraclelinux"
            | "parrot"
            | "pop"
            | "popos"
            | "postmarketos"
            | "proxmox"
            | "puppy"
            | "puppylinux"
            | "qubes"
            | "qubesos"
            | "raspbian"
            | "raspberry"
            | "redhat"
            | "redos"
            | "rhel"
            | "rocky"
            | "slackware"
            | "solaris"
            | "solus"
            | "star"
            | "steamos"
            | "steamdeck"
            | "swagarch"
            | "tails"
            | "templeos"
            | "trisquel"
            | "truenas"
            | "tux"
            | "ublinux"
            | "ubuntu"
            | "vanilla"
            | "vanillaos"
            | "void"
            | "whonix"
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
            s if s.contains("endeavour") => (include_str!("../assets/mini/e/endeavouros.txt"), |s| s.purple().bold()),
            s if s.contains("artix")    => (include_str!("../assets/mini/a/artix.txt"), |s| s.blue().bold()),
            s if s.contains("garuda")   => (include_str!("../assets/mini/g/garuda.txt"), |s| s.red().bold()),
            s if s.contains("asahi")    => (include_str!("../assets/mini/a/asahi.txt"), |s| s.red().bold()),
            s if s.contains("arch")    => (include_str!("../assets/mini/a/arch.txt"), |s| s.blue().bold()),

            // Debian Based
            s if s.contains("devuan")   => (include_str!("../assets/mini/d/devuan.txt"), |s| s.purple().bold()),
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
            s if s.contains("oracle")   => (include_str!("../assets/mini/o/oracle.txt"), |s| s.red().bold()),
            s if s.contains("amazon") || s == "amzn" => (include_str!("../assets/mini/a/amazon.txt"), |s| s.yellow().bold()),

            // BSD & Storage
            s if s.contains("ghostbsd")    => (include_str!("../assets/mini/g/ghostbsd.txt"), |s| s.cyan().bold()),
            s if s.contains("midnightbsd") => (include_str!("../assets/mini/m/midnightbsd.txt"), |s| s.blue().bold()),
            s if s.contains("nomadbsd")    => (include_str!("../assets/mini/n/nomadbsd.txt"), |s| s.blue().bold()),
            s if s.contains("dragonfly")   => (include_str!("../assets/mini/d/dragonfly.txt"), |s| s.red().bold()),
            s if s.contains("truenas")     => (include_str!("../assets/mini/t/truenas.txt"), |s| s.cyan().bold()),
            s if s.contains("freebsd")     => (include_str!("../assets/mini/f/freebsd.txt"), |s| s.red().bold()),
            s if s.contains("netbsd")      => (include_str!("../assets/mini/n/netbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("openbsd")     => (include_str!("../assets/mini/o/openbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("solaris")     => (include_str!("../assets/mini/s/solaris.txt"), |s| s.red().bold()),
            s if s == "bsd" || s.contains("bsd") => (include_str!("../assets/mini/b/bsd.txt"), |s| s.red().bold()),

            // Independent & Others
            s if s.contains("athena")     => (include_str!("../assets/mini/a/athenaos.txt"), |s| s.blue().bold()),
            s if s.contains("azure")      => (include_str!("../assets/mini/a/azurelinux.txt"), |s| s.blue().bold()),
            s if s.contains("bedrock")    => (include_str!("../assets/mini/b/bedrock.txt"), |s| s.white().bold()),
            s if s.contains("cosmic")     => (include_str!("../assets/mini/c/cosmic.txt"), |s| s.blue().bold()),
            s if s.contains("femboy")     => (include_str!("../assets/mini/f/femboyos.txt"), |s| s.purple().bold()),
            s if s.contains("gnome")      => (include_str!("../assets/mini/g/gnome.txt"), |s| s.blue().bold()),
            s if s.contains("gnu")        => (include_str!("../assets/mini/g/gnu.txt"), |s| s.yellow().bold()),
            s if s.contains("graphene")   => (include_str!("../assets/mini/g/grapheneos.txt"), |s| s.cyan().bold()),
            s if s.contains("kdelinux") || s == "kde" => (include_str!("../assets/mini/k/kdelinux.txt"), |s| s.cyan().bold()),
            s if s.contains("kernelos") || s == "kernel" => (include_str!("../assets/mini/k/kernelos.txt"), |s| s.blue().bold()),
            s if s.contains("minix")      => (include_str!("../assets/mini/m/minix.txt"), |s| s.blue().bold()),
            s if s.contains("obsidian")   => (include_str!("../assets/mini/o/obsidianos.txt"), |s| s.purple().bold()),
            s if s.contains("omarchy")    => (include_str!("../assets/mini/o/omarchy.txt"), |s| s.green().bold()),
            s if s.contains("openwrt")    => (include_str!("../assets/mini/o/openwrt.txt"), |s| s.cyan().bold()),
            s if s.contains("postmarketos") => (include_str!("../assets/mini/p/postmarketos.txt"), |s| s.green().bold()),
            s if s.contains("redos")      => (include_str!("../assets/mini/r/redos.txt"), |s| s.red().bold()),
            s if s == "star" || s.contains("staros") => (include_str!("../assets/mini/s/star.txt"), |s| s.yellow().bold()),
            s if s.contains("swagarch")   => (include_str!("../assets/mini/s/swagarch.txt"), |s| s.blue().bold()),
            s if s.contains("templeos")   => (include_str!("../assets/mini/t/templeos.txt"), |s| s.white().bold()),
            s if s.contains("trisquel")   => (include_str!("../assets/mini/t/trisquel.txt"), |s| s.blue().bold()),
            s if s.contains("ublinux")    => (include_str!("../assets/mini/u/ublinux.txt"), |s| s.blue().bold()),
            s if s.contains("nobara")   => (include_str!("../assets/mini/n/nobara.txt"), |s| s.purple().bold()),
            s if s.contains("bazzite")  => (include_str!("../assets/mini/b/bazzite.txt"), |s| s.purple().bold()),
            s if s.contains("whonix")   => (include_str!("../assets/mini/w/whonix.txt"), |s| s.cyan().bold()),
            s if s.contains("mageia")   => (include_str!("../assets/mini/m/mageia.txt"), |s| s.cyan().bold()),
            s if s.contains("antix")    => (include_str!("../assets/mini/a/antix.txt"), |s| s.blue().bold()),
            s if s.contains("kaos")     => (include_str!("../assets/mini/k/kaos.txt"), |s| s.blue().bold()),
            s if s.contains("vanilla")  => (include_str!("../assets/mini/v/vanilla.txt"), |s| s.yellow().bold()),
            s if s.contains("bodhi")    => (include_str!("../assets/mini/b/bodhi.txt"), |s| s.green().bold()),
            s if s.contains("guix")     => (include_str!("../assets/mini/g/guix.txt"), |s| s.yellow().bold()),
            s if s.contains("chimera")  => (include_str!("../assets/mini/c/chimera.txt"), |s| s.red().bold()),
            s if s.contains("clearlinux") || s.contains("clear-linux") || s == "clear" => (include_str!("../assets/mini/c/clearlinux.txt"), |s| s.blue().bold()),
            s if s.contains("qubes")    => (include_str!("../assets/mini/q/qubes.txt"), |s| s.blue().bold()),
            s if s.contains("puppy")    => (include_str!("../assets/mini/p/puppy.txt"), |s| s.blue().bold()),
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
            s if s.contains("endeavour") => (include_str!("../assets/normal/e/endeavouros.txt"), |s| s.purple().bold()),
            s if s.contains("artix")    => (include_str!("../assets/normal/a/artix.txt"), |s| s.blue().bold()),
            s if s.contains("garuda")   => (include_str!("../assets/normal/g/garuda.txt"), |s| s.red().bold()),
            s if s.contains("asahi")    => (include_str!("../assets/normal/a/asahi.txt"), |s| s.red().bold()),
            s if s.contains("arch")    => (include_str!("../assets/normal/a/arch.txt"), |s| s.blue().bold()),

            // Debian Based
            s if s.contains("devuan")   => (include_str!("../assets/normal/d/devuan.txt"), |s| s.purple().bold()),
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
            s if s.contains("oracle")   => (include_str!("../assets/normal/o/oracle.txt"), |s| s.red().bold()),
            s if s.contains("amazon") || s == "amzn" => (include_str!("../assets/normal/a/amazon.txt"), |s| s.yellow().bold()),

            // BSD & Storage
            s if s.contains("ghostbsd")    => (include_str!("../assets/normal/g/ghostbsd.txt"), |s| s.cyan().bold()),
            s if s.contains("midnightbsd") => (include_str!("../assets/normal/m/midnightbsd.txt"), |s| s.blue().bold()),
            s if s.contains("nomadbsd")    => (include_str!("../assets/normal/n/nomadbsd.txt"), |s| s.blue().bold()),
            s if s.contains("dragonfly")   => (include_str!("../assets/normal/d/dragonfly.txt"), |s| s.red().bold()),
            s if s.contains("truenas")     => (include_str!("../assets/normal/t/truenas.txt"), |s| s.cyan().bold()),
            s if s.contains("freebsd")     => (include_str!("../assets/normal/f/freebsd.txt"), |s| s.red().bold()),
            s if s.contains("netbsd")      => (include_str!("../assets/normal/n/netbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("openbsd")     => (include_str!("../assets/normal/o/openbsd.txt"), |s| s.yellow().bold()),
            s if s.contains("solaris")     => (include_str!("../assets/normal/s/solaris.txt"), |s| s.red().bold()),
            s if s == "bsd" || s.contains("bsd") => (include_str!("../assets/normal/b/bsd.txt"), |s| s.red().bold()),

            // Independent & Others
            s if s.contains("athena")     => (include_str!("../assets/normal/a/athenaos.txt"), |s| s.blue().bold()),
            s if s.contains("azure")      => (include_str!("../assets/normal/a/azurelinux.txt"), |s| s.blue().bold()),
            s if s.contains("bedrock")    => (include_str!("../assets/normal/b/bedrock.txt"), |s| s.white().bold()),
            s if s.contains("cosmic")     => (include_str!("../assets/normal/c/cosmic.txt"), |s| s.blue().bold()),
            s if s.contains("femboy")     => (include_str!("../assets/normal/f/femboyos.txt"), |s| s.purple().bold()),
            s if s.contains("gnome")      => (include_str!("../assets/normal/g/gnome.txt"), |s| s.blue().bold()),
            s if s.contains("gnu")        => (include_str!("../assets/normal/g/gnu.txt"), |s| s.yellow().bold()),
            s if s.contains("graphene")   => (include_str!("../assets/normal/g/grapheneos.txt"), |s| s.cyan().bold()),
            s if s.contains("kdelinux") || s == "kde" => (include_str!("../assets/normal/k/kdelinux.txt"), |s| s.cyan().bold()),
            s if s.contains("kernelos") || s == "kernel" => (include_str!("../assets/normal/k/kernelos.txt"), |s| s.blue().bold()),
            s if s.contains("minix")      => (include_str!("../assets/normal/m/minix.txt"), |s| s.blue().bold()),
            s if s.contains("obsidian")   => (include_str!("../assets/normal/o/obsidianos.txt"), |s| s.purple().bold()),
            s if s.contains("omarchy")    => (include_str!("../assets/normal/o/omarchy.txt"), |s| s.green().bold()),
            s if s.contains("openwrt")    => (include_str!("../assets/normal/o/openwrt.txt"), |s| s.cyan().bold()),
            s if s.contains("postmarketos") => (include_str!("../assets/normal/p/postmarketos.txt"), |s| s.green().bold()),
            s if s.contains("redos")      => (include_str!("../assets/normal/r/redos.txt"), |s| s.red().bold()),
            s if s == "star" || s.contains("staros") => (include_str!("../assets/normal/s/star.txt"), |s| s.yellow().bold()),
            s if s.contains("swagarch")   => (include_str!("../assets/normal/s/swagarch.txt"), |s| s.blue().bold()),
            s if s.contains("templeos")   => (include_str!("../assets/normal/t/templeos.txt"), |s| s.white().bold()),
            s if s.contains("trisquel")   => (include_str!("../assets/normal/t/trisquel.txt"), |s| s.blue().bold()),
            s if s.contains("ublinux")    => (include_str!("../assets/normal/u/ublinux.txt"), |s| s.blue().bold()),
            s if s.contains("nobara")   => (include_str!("../assets/normal/n/nobara.txt"), |s| s.purple().bold()),
            s if s.contains("bazzite")  => (include_str!("../assets/normal/b/bazzite.txt"), |s| s.purple().bold()),
            s if s.contains("whonix")   => (include_str!("../assets/normal/w/whonix.txt"), |s| s.cyan().bold()),
            s if s.contains("mageia")   => (include_str!("../assets/normal/m/mageia.txt"), |s| s.cyan().bold()),
            s if s.contains("antix")    => (include_str!("../assets/normal/a/antix.txt"), |s| s.blue().bold()),
            s if s.contains("kaos")     => (include_str!("../assets/normal/k/kaos.txt"), |s| s.blue().bold()),
            s if s.contains("vanilla")  => (include_str!("../assets/normal/v/vanilla.txt"), |s| s.yellow().bold()),
            s if s.contains("bodhi")    => (include_str!("../assets/normal/b/bodhi.txt"), |s| s.green().bold()),
            s if s.contains("guix")     => (include_str!("../assets/normal/g/guix.txt"), |s| s.yellow().bold()),
            s if s.contains("chimera")  => (include_str!("../assets/normal/c/chimera.txt"), |s| s.red().bold()),
            s if s.contains("clearlinux") || s.contains("clear-linux") || s == "clear" => (include_str!("../assets/normal/c/clearlinux.txt"), |s| s.blue().bold()),
            s if s.contains("qubes")    => (include_str!("../assets/normal/q/qubes.txt"), |s| s.blue().bold()),
            s if s.contains("puppy")    => (include_str!("../assets/normal/p/puppy.txt"), |s| s.blue().bold()),
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
    fn test_new_distros_logos() {
        let distros = [
            "nobara", "devuan", "mageia", "antix", "kaos",
            "vanilla", "bodhi", "guix", "chimera", "clearlinux",
            "dragonfly", "oracle", "amazon", "qubes", "puppy",
            "bazzite", "whonix",
            "athenaos", "azurelinux", "bedrock", "bsd", "cosmic",
            "endeavouros", "femboyos", "ghostbsd", "gnome", "gnu",
            "grapheneos", "kdelinux", "kernelos", "midnightbsd",
            "minix", "nomadbsd", "obsidianos", "omarchy", "openwrt",
            "postmarketos", "redos", "solaris", "star", "swagarch",
            "templeos", "trisquel", "ublinux",
        ];
        for d in distros {
            assert!(is_valid_logo(d), "is_valid_logo failed for {d}");
            let (normal_lines, normal_w, _) = get_logo(false, Some(d));
            assert!(!normal_lines.is_empty(), "normal logo empty for {d}");
            assert!(normal_w > 0, "normal logo width 0 for {d}");

            let (mini_lines, mini_w, _) = get_logo(true, Some(d));
            assert!(!mini_lines.is_empty(), "mini logo empty for {d}");
            assert!(mini_w > 0, "mini logo width 0 for {d}");
        }
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