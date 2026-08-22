use colored::Colorize;
use std::env;
use chrono::Local;
use std::fmt::Write;
use std::fs;
use colored::{ColoredString};
use crate::sysinfo::combine::DisplayOptions;

pub fn other_info(opts: &DisplayOptions, buf: &mut String, c :fn(&str) -> ColoredString) {
    // --- Other Info ---
    if !opts.other {
        return;
    }

    let _ = writeln!(buf,"{}", "--- Other Info ---".bold().cyan());

    // give link functions de_wm_check for variables lines
    de_wm_check(buf, c);

    // Functions de_wm_check
    fn de_wm_check(buf: &mut String, c:fn(&str) -> ColoredString) {
        // Desktop Environment / Window Manager
        let desktop = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string());
        let _ = writeln!(buf,"{}: {}", c("DE"), desktop);
    }


    fn get_shell(buf: &mut String, c:fn(&str) -> ColoredString) {
        let shell_name = if let Ok(shell_path) = env::var("SHELL") {
            shell_path
                .split('/')
                .last()
                .unwrap_or("Unknown")
                .to_string()
        } else {
            env::var("ComSpec")
                .map(|p| p.split('\\').last().unwrap_or("cmd.exe").to_string())
                .unwrap_or_else(|_| "Unknown".to_string())
        };

        let _ = writeln!(buf,"{}: {}", c("Shell"), shell_name);
    }

    get_shell(buf, c);

    fn system_time(buf: &mut String, c:fn(&str) -> ColoredString) {
        let now = Local::now();
        now.format("%H:%M").to_string();
        let _ = writeln!(buf,"{}: {}", c("Locale Time"), now.format("%H:%M").to_string());
    }


    // give link functions battery for variables lines
    battery(buf, c);

    // Functions battery
    fn battery(buf: &mut String, c: fn(&str) -> ColoredString) {
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
            use std::process::Command;
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
            use std::process::Command;
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
            use std::process::Command;
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
            use std::process::Command;
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

        let _ = writeln!(buf,"{}: {}", c("Battery"), battery);


    }
    system_time(buf, c);
}