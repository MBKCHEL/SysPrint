use colored::Colorize;
use sysinfo::{System};
use crate::sysinfo::combine::DisplayOptions;

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
pub fn system_info(opts: &DisplayOptions, lines: &mut Vec<String>){

    if opts.system {
        lines.push(format!("{}", "--- System INFO ---".bold().cyan()));


        os_name(lines);

        // OS_name
        fn os_name(lines: &mut Vec<String>){

            lines.push(format!(
                "{}: {}",
                "OS".bold(),
                System::name().unwrap_or_default()
            ));

        }

        os_version(lines);

        // OS_version
        fn os_version(lines: &mut Vec<String>){
            lines.push(format!(
                "{}: {}",
                "OS Version".bold(),
                System::os_version().unwrap_or_default()
            ));
        }

        host(lines);

        // Host name
        fn host(lines: &mut Vec<String>){
            lines.push(format!(
                "{}: {}",
                "Host".bold(),
                System::host_name().unwrap_or_default()
            ));
        }

        //Uptime
        lines.push(format!(
            "{}: {}",
            "Uptime".bold(),
            format_uptime(System::uptime())
        ));
    }
}