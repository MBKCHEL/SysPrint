use crate::logos::parse_color;
use crate::sysinfo::combine::DisplayOptions;
use colored::Colorize;
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
pub fn system_info(opts: &DisplayOptions, buf: &mut String, c: &str) {
    if !opts.system.enabled {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", "--- System INFO ---".bold().cyan());
    }
    if opts.system.os_name {
        os_name(buf, c);
    }
    if opts.system.kernel {
        check_kernel(buf, c);
    }
    if opts.compact_mode {
        return;
    }
    if opts.system.os_version {
        os_version(buf, c);
    }
    if opts.system.init {
        init_info(buf, c);
    }
    if opts.system.host {
        host(buf, c);
    }
    if opts.system.user {
        user_info(buf, c);
    }
    if opts.system.uptime {
        let _ = writeln!(buf, "{}: {}", parse_color("Uptime", c), format_uptime(System::uptime()));
    }
}

// OS_name
fn os_name(buf: &mut String, c: &str) {
    let _ = writeln!(buf, "{}: {}", parse_color("OS", c), System::name().unwrap_or_default());
}

// OS_version
fn os_version(buf: &mut String, c: &str) {
    let _ = writeln!(
        buf,
        "{}: {}",
        parse_color("OS Version", c),
        System::os_version().unwrap_or_default()
    );
}

// Host name
fn host(buf: &mut String, c: &str) {
    let _ = writeln!(
        buf,
        "{}: {}",
        parse_color("Host", c),
        System::host_name().unwrap_or_default()
    );
}

fn check_kernel(buf: &mut String, c: &str) {
    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let _ = writeln!(buf, "{}: {}", parse_color("Kernel", c), kernel);
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

pub fn init_info(buf: &mut String, c: &str) {
    let init = get_init_system();
    let _ = writeln!(buf, "{}: {}", parse_color("Init", c), init);
}

pub fn user_info(buf: &mut String, c: &str) {
    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    let _ = writeln!(buf, "{}: {}", parse_color("User", c), username);
}