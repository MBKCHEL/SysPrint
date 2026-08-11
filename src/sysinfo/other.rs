use colored::Colorize;
use std::env;
use std::fs;
use crate::sysinfo::combine::DisplayOptions;

pub fn other_info(opts: &DisplayOptions, mut lines: &mut Vec<String>) {
    // --- Other Info ---
    opts.other;
    lines.push(format!("{}", "--- Other Info ---".bold().cyan()));

    // give link functions de_wm_check for variables lines
    de_wm_check(&mut lines);

    // Functions de_wm_check
    fn de_wm_check(lines: &mut Vec<String>){
        // Desktop Environment / Window Manager
        let desktop = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string());
        lines.push(format!("{}: {}", "DE/WM".bold(), desktop));
    }

    // give link functions battery for variables lines
    battery(&mut lines);

    // Functions battery
    fn battery(lines: &mut Vec<String>){
        let mut battery = "N/A (Desktop)".to_string();

        // Linux battery
        #[cfg(target_os = "linux")]
        {


            if let (Ok(cap), Ok(stat)) = (
                fs::read_to_string("/sys/class/power_supply/BAT0/capacity"),
                fs::read_to_string("/sys/class/power_supply/BAT0/status"),
            ) {
                battery = format!("{}% [{}]", cap.trim(), stat.trim());
            } else if let (Ok(cap), Ok(stat)) = (
                fs::read_to_string("/sys/class/power_supply/BAT1/capacity"),
                fs::read_to_string("/sys/class/power_supply/BAT1/status"),
            ) {
                battery = format!("{}% [{}]", cap.trim(), stat.trim());
            }
        }

        // FreeBSD battery
        #[cfg(target_os = "freebsd")]
        {
            let output = Command::new("sysctl")
                .arg("-n")
                .arg("hw.acpi.battery.life")
                .output();

            if let Ok(out) = output {
                let cap = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !cap.is_empty() {
                    battery = format!("{}%", cap);
                }
            }
        }
        // OpenBSD battery
        #[cfg(target_os = "openbsd")]
        {
            // apm -l
            let output = Command::new("apm").arg("-l").output();

            if let Ok(out) = output {
                let cap = String::from_utf8_lossy(&out.stdout).trim().to_string();
                // 255 в apm означает "батарея не найдена / ПК от сети"
                if !cap.is_empty() && cap != "255" {
                    battery = format!("{}%", cap);
                }
            }
        }
        // NetBSD battery
        #[cfg(target_os = "netbsd")]
        {
            //envstat
            let output = Command::new("envstat").args(["-s", "bat0:charge"]).output();

            if let Ok(out) = output {
                let text = String::from_utf8_lossy(&out.stdout);
                if let Some(line) = text.lines().find(|l| l.contains("%")) {
                    if let Some(val) = line.split('(').next() {
                        let cleaned = val.replace("charge:", "").replace("%", "");
                        let trimmed = cleaned.trim();
                        if !trimmed.is_empty() {
                            battery = format!("{}%", trimmed);
                        }
                    }
                }
            }
        }
        // Windows battery
        #[cfg(windows)]
        {
            let output = Command::new("wmic")
                .args(["path", "Win32_Battery", "get", "EstimatedChargeRemaining"])
                .output();

            if let Ok(out) = output {
                let text = String::from_utf8_lossy(&out.stdout);
                if let Some(cap) = text.lines().nth(1) {
                    let trimmed = cap.trim();
                    if !trimmed.is_empty() {
                        battery = format!("{}%", trimmed);
                    }
                }
            }
        }

        lines.push(format!("{}: {}", "Battery".bold(), battery));
    }
}