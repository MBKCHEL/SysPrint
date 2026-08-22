use colored::Colorize;
use sysinfo::{System};
use std::fmt::Write;
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
pub fn system_info(opts: &DisplayOptions, buf: &mut String){

    if !opts.system {
        return;
    }

    let _ = writeln!(buf, "{}", "--- System INFO ---".bold().cyan());


        os_name(buf);

        // OS_name
        fn os_name(buf: &mut String){

            let _ = writeln!(buf,
                "{}: {}",
                "OS".bold(),
                System::name().unwrap_or_default()
            );

        }

        os_version(buf);

        // OS_version
        fn os_version(buf: &mut String){
            let _ = writeln!(buf,
                "{}: {}",
                "OS Version".bold(),
                System::os_version().unwrap_or_default()
            );
        }

        host(buf);

        // Host name
        fn host(buf: &mut String){
            let _ = writeln!(buf,
                "{}: {}",
                "Host".bold(),
                System::host_name().unwrap_or_default()
            );
        }

        //Uptime
         let _ = writeln!(buf,
            "{}: {}",
            "Uptime".bold(),
            format_uptime(System::uptime())
        );
    }
