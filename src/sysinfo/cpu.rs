use colored::Colorize;
use sysinfo::{Components, System};
use crate::sysinfo::combine::DisplayOptions;

// --- CPU INFO ---
pub fn cpu_info(opts: &DisplayOptions, lines: &mut Vec<String>, sys: &System) {
    if !opts.cpu {
        return;
    }

    lines.push(format!("{}", "--- CPU INFO ---".bold().cyan()));

    ggz_and_name_cpu(lines, sys);
    cpu_usage(lines, sys);
    cpu_temperature(lines);
    cpu_cores_and_threads(lines, sys);
    cpu_arch(lines);

    fn ggz_and_name_cpu(lines: &mut Vec<String>, sys: &System) {
        let cpus = sys.cpus();
        if let Some(cpu) = cpus.first() {
            lines.push(format!("{}: {}", "CPU name".bold(), cpu.brand().trim()));
            let freq_ghz = cpu.frequency() as f64 / 1000.0;
            lines.push(format!("{}: {:.2} GHz", "GHz".bold(), freq_ghz));
        } else {
            lines.push("CPU: Unknown".to_string());
        }
    }

    fn cpu_usage(lines: &mut Vec<String>, sys: &System) {
        lines.push(format!(
            "{}: {:.1}%",
            "CPU Usage".bold(),
            sys.global_cpu_usage()
        ));
    }

    fn cpu_temperature(lines: &mut Vec<String>) {
        let components = Components::new_with_refreshed_list();
        let cpu_temp = components.iter().find_map(|comp| {
            let label = comp.label().to_lowercase();
            if label.contains("cpu")
                || label.contains("core")
                || label.contains("package")
                || label.contains("k10temp")
                || label.contains("zenpower")
            {
                comp.temperature()
            } else {
                None
            }
        });

        if let Some(temp) = cpu_temp {
            lines.push(format!("{}: {:.1}°C", "CPU Temp".bold(), temp));
        }
    }

    fn cpu_cores_and_threads(lines: &mut Vec<String>, sys: &System) {
        lines.push(format!(
            "{}: {}",
            "Cores".bold(),
            System::physical_core_count().unwrap_or(0)
        ));
        let cpus = sys.cpus();
        lines.push(format!("{}: {}", "Threads".bold(), cpus.len()));
    }

    fn cpu_arch(lines: &mut Vec<String>) {
        lines.push(format!(
            "{}: {}",
            "Architecture".bold(),
            std::env::consts::ARCH,
        ));
    }
}