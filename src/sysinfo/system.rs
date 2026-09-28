use crate::sysinfo::combine::DisplayOptions;
use colored::ColoredString;
use std::env;
use std::fmt::Write;
use std::fs;
use std::path::Path;
use sysinfo::System;

// Uptime functions
fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

// --- SYSTEM INFO ---
pub fn system_info(
    opts: &DisplayOptions,
    buf: &mut String,
    sys: &System,
    c: fn(&str) -> ColoredString,
) {
    if !opts.system {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", opts.format_header("--- System INFO ---", c));
    }

    if opts.os {
        os_name(buf, c);
    }
    if opts.kernel {
        check_kernel(buf, c);
    }
    if opts.compact_mode {
        return;
    }
    if opts.os_version {
        os_version(buf, c);
    }
    if opts.init {
        init_info(buf, c);
    }
    if opts.host {
        host(buf, c);
    }
    if opts.user {
        user_info(buf, c);
    }

    // OS_name
    fn os_name(buf: &mut String, c: fn(&str) -> ColoredString) {
        let _ = writeln!(buf, "{}: {}", c("OS"), System::name().unwrap_or_default());
    }

    // OS_version
    fn os_version(buf: &mut String, c: fn(&str) -> ColoredString) {
        let version = System::os_version()
            .filter(|v| !v.trim().is_empty())
            .or_else(|| {
                #[cfg(target_os = "linux")]
                {
                    if let Ok(content) = fs::read_to_string("/etc/os-release") {
                        for line in content.lines() {
                            if let Some(rest) = line.strip_prefix("VERSION_ID=")
                                .or_else(|| line.strip_prefix("BUILD_ID="))
                                .or_else(|| line.strip_prefix("VERSION="))
                            {
                                let val = rest.trim().trim_matches('"').trim_matches('\'');
                                if !val.is_empty() {
                                    return Some(val.to_string());
                                }
                            }
                        }
                    }
                }
                None
            });

        if let Some(v) = version {
            let _ = writeln!(buf, "{}: {}", c("OS Version"), v);
        }
    }

    // Host name
    fn host(buf: &mut String, c: fn(&str) -> ColoredString) {
        let _ = writeln!(
            buf,
            "{}: {}",
            c("Host"),
            System::host_name().unwrap_or_default()
        );
    }

    fn check_kernel(buf: &mut String, c: fn(&str) -> ColoredString) {
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

        let _ = writeln!(buf, "{}: {}", c("Kernel"), kernel);
    }

    fn init_info(buf: &mut String, c: fn(&str) -> ColoredString) {
        let init = get_init_system();
        let _ = writeln!(buf, "{}: {}", c("Init"), init);
    }

    pub fn user_info(buf: &mut String, c: fn(&str) -> ColoredString) {
        let username = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());

        let _ = writeln!(buf, "{}: {}", c("User"), username);
    }

    // Uptime
    if opts.uptime {
        let _ = writeln!(buf, "{}: {}", c("Uptime"), format_uptime(System::uptime()));
    }

    // Load Average
    if opts.load_avg {
        let load = System::load_average();
        if load.one > 0.0 || load.five > 0.0 || load.fifteen > 0.0 {
            let _ = writeln!(
                buf,
                "{}: {:.2}, {:.2}, {:.2}",
                c("Load Avg"),
                load.one,
                load.five,
                load.fifteen
            );
        }
    }

    // Processes
    if opts.processes {
        let stats = get_process_stats(sys, opts.fast_mode);
        if stats.total > 0 {
            if stats.running > 0 || stats.zombie > 0 {
                let _ = writeln!(
                    buf,
                    "{}: {} (running: {}, zombie: {})",
                    c("Processes"),
                    stats.total,
                    stats.running,
                    stats.zombie
                );
            } else {
                let _ = writeln!(buf, "{}: {}", c("Processes"), stats.total);
            }
        }
    }
}

pub fn get_init_system() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(comm) = fs::read_to_string("/proc/1/comm") {
            let name = comm.trim();
            match name {
                "systemd" => return "systemd".to_string(),
                "init" | "sysvinit" => {
                    if Path::new("/run/openrc").exists() || Path::new("/etc/openrc").exists() {
                        return "OpenRC".to_string();
                    }
                    if Path::new("/etc/runit").exists() || Path::new("/run/runit").exists() {
                        return "runit".to_string();
                    }
                    return "SysVinit".to_string();
                }
                "runit" | "runit-init" => return "runit".to_string(),
                "dinit" => return "dinit".to_string(),
                "s6-svscan" => return "s6".to_string(),
                _ => return name.to_string(),
            }
        }

        if Path::new("/run/systemd/system").exists() {
            return "systemd".to_string();
        } else if Path::new("/run/openrc").exists() || Path::new("/etc/openrc").exists() {
            return "OpenRC".to_string();
        } else if Path::new("/etc/runit").exists() || Path::new("/run/runit").exists() {
            return "runit".to_string();
        } else if Path::new("/sbin/dinit").exists() {
            return "dinit".to_string();
        }
    }

    if cfg!(target_os = "freebsd")
        || cfg!(target_os = "openbsd")
        || cfg!(target_os = "netbsd")
    {
        "BSD init".to_string()
    } else if cfg!(target_os = "macos") {
        "launchd".to_string()
    } else if cfg!(windows) {
        "SMSS".to_string()
    } else {
        "Unknown".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProcessStats {
    pub total: usize,
    pub running: usize,
    pub zombie: usize,
}

pub fn parse_stat_state(content: &str) -> Option<char> {
    let idx = content.rfind(')')?;
    let rest = content[idx + 1..].trim_start();
    rest.chars().next()
}

pub fn parse_loadavg_processes(content: &str) -> Option<(usize, usize)> {
    let mut parts = content.split_whitespace();
    let fourth = parts.nth(3)?;
    let mut slash_parts = fourth.split('/');
    let running = slash_parts.next()?.parse::<usize>().ok()?;
    let total = slash_parts.next()?.parse::<usize>().ok()?;
    Some((running, total))
}

pub fn get_process_stats(sys: &System, fast_mode: bool) -> ProcessStats {
    #[cfg(target_os = "linux")]
    {
        if fast_mode
            && let Ok(content) = fs::read_to_string("/proc/loadavg")
            && let Some((running, total)) = parse_loadavg_processes(&content)
        {
            return ProcessStats {
                total,
                running,
                zombie: 0,
            };
        }

        if let Ok(entries) = fs::read_dir("/proc") {
            let mut total = 0;
            let mut running = 0;
            let mut zombie = 0;

            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_bytes = name.as_encoded_bytes();
                if !name_bytes.is_empty() && name_bytes.iter().all(|b| b.is_ascii_digit()) {
                    total += 1;
                    if !fast_mode {
                        let stat_path = entry.path().join("stat");
                        if let Ok(content) = fs::read_to_string(&stat_path)
                            && let Some(state) = parse_stat_state(&content)
                        {
                            match state {
                                'R' => running += 1,
                                'Z' => zombie += 1,
                                _ => {}
                            }
                        }
                    }
                }
            }

            if total > 0 {
                return ProcessStats { total, running, zombie };
            }
        }
    }

    ProcessStats {
        total: sys.processes().len(),
        running: 0,
        zombie: 0,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_uptime() {
        assert_eq!(format_uptime(45), "0m");
        assert_eq!(format_uptime(120), "2m");
        assert_eq!(format_uptime(3660), "1h 1m");
        assert_eq!(format_uptime(90060), "1d 1h 1m");
    }

    #[test]
    fn test_parse_stat_state() {
        let stat_running = "1234 (bash) R 1000 1234 1000 34816 1234";
        assert_eq!(parse_stat_state(stat_running), Some('R'));

        let stat_zombie = "5678 (defunct_proc) Z 1 5678 1 0 -1";
        assert_eq!(parse_stat_state(stat_zombie), Some('Z'));

        let stat_parens = "9999 (complex (name)) S 1 9999 1 0 -1";
        assert_eq!(parse_stat_state(stat_parens), Some('S'));
    }

    #[test]
    fn test_parse_loadavg_processes() {
        let sample = "0.75 0.79 0.78 3/214 3482\n";
        assert_eq!(parse_loadavg_processes(sample), Some((3, 214)));

        let invalid = "invalid data";
        assert_eq!(parse_loadavg_processes(invalid), None);
    }
}


