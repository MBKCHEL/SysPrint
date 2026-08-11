use colored::Colorize;
use sysinfo::{System};
use crate::sysinfo::combine::DisplayOptions;



// --- SYSTEM INFO ---
pub fn system_info(opts: &DisplayOptions, lines: &mut Vec<String>){

    if opts.system {
        lines.push(format!("{}", "--- System INFO ---".bold().cyan()));


        os_name(lines);
        fn os_name(lines: &mut Vec<String>){

            lines.push(format!(
                "{}: {}",
                "OS".bold(),
                System::name().unwrap_or_default()
            ));

        }

        os_version(lines);

        fn os_version(lines: &mut Vec<String>){
            lines.push(format!(
                "{}: {}",
                "OS Version".bold(),
                System::os_version().unwrap_or_default()
            ));
        }

        host(lines);

        fn host(lines: &mut Vec<String>){
            lines.push(format!(
                "{}: {}",
                "Host".bold(),
                System::host_name().unwrap_or_default()
            ));
        }
    }
}