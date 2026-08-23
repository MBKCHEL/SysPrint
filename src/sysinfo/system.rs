use crate::sysinfo::combine::DisplayOptions;
use colored::ColoredString;
use colored::Colorize;
use std::env;
use std::fmt::Write;
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
pub fn system_info(opts: &DisplayOptions, buf: &mut String, c: fn(&str) -> ColoredString) {
    if !opts.system {
        return;
    }

    let _ = writeln!(buf, "{}", "--- System INFO ---".bold().cyan());

    os_name(buf, c);

    // OS_name
    fn os_name(buf: &mut String, c: fn(&str) -> ColoredString) {
        let _ = writeln!(buf, "{}: {}", c("OS"), System::name().unwrap_or_default());
    }

    os_version(buf, c);

    // OS_version
    fn os_version(buf: &mut String, c: fn(&str) -> ColoredString) {
        let _ = writeln!(
            buf,
            "{}: {}",
            c("OS Version"),
            System::os_version().unwrap_or_default()
        );
    }

    host(buf, c);
    user_info(buf, c);

    // Host name
    fn host(buf: &mut String, c: fn(&str) -> ColoredString) {
        let _ = writeln!(
            buf,
            "{}: {}",
            c("Host"),
            System::host_name().unwrap_or_default()
        );
    }

    pub fn user_info(buf: &mut String, c: fn(&str) -> ColoredString) {
        let username = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());

        let _ = writeln!(buf, "{}: {}", c("User"), username);
    }

    //Uptime
    let _ = writeln!(buf, "{}: {}", c("Uptime"), format_uptime(System::uptime()));
}
