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

    pub fn get_init_system() -> String {
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
            "systemd".to_string()
        } else if Path::new("/run/openrc").exists() || Path::new("/etc/openrc").exists() {
            "OpenRC".to_string()
        } else if Path::new("/etc/runit").exists() || Path::new("/run/runit").exists() {
            "runit".to_string()
        } else if Path::new("/sbin/dinit").exists() {
            "dinit".to_string()
        } else if cfg!(target_os = "freebsd")
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

    pub fn init_info(buf: &mut String, c: fn(&str) -> ColoredString) {
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

    // Processes
    if opts.processes {
        let proc_count = get_process_count(sys);
        if proc_count > 0 {
            let _ = writeln!(buf, "{}: {}", c("Processes"), proc_count);
        }
    }
}

fn get_process_count(sys: &System) -> usize {
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = fs::read_dir("/proc") {
            let count = entries
                .flatten()
                .filter(|e| e.file_name().to_string_lossy().chars().all(|c| c.is_ascii_digit()))
                .count();
            if count > 0 {
                return count;
            }
        }
    }
    sys.processes().len()
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
}


