use std::fmt::Write;
use colored::{ColoredString};
use sysinfo::{Components, System};
use crate::sysinfo::combine::DisplayOptions;
use colored::Colorize;

pub fn cpu_info(opts: &DisplayOptions, buf: &mut String, sys: &System, c :fn(&str) -> ColoredString) {
    if !opts.cpu {
        return;
    }

    let _ = writeln!(buf, "{}", "--- CPU INFO ---".bold().cyan());

    ggz_and_name_cpu(buf, sys, c);
    cpu_usage(buf, sys, c);
    cpu_temperature(buf, c);
    cpu_cores_and_threads(buf, sys, c);
    cpu_arch(buf, c);

    fn ggz_and_name_cpu(buf: &mut String, sys: &System, c :fn(&str) -> ColoredString) {
        let cpus = sys.cpus();
        if let Some(cpu) = cpus.first() {
            let _ = writeln!(buf, "{}: {}", c("CPU name"), cpu.brand().trim());
            let freq_ghz = cpu.frequency() as f64 / 1000.0;
            let _ = writeln!(buf, "{}: {:.2} GHz", c("GHz"), freq_ghz);
        } else {
            let _ = writeln!(buf, "CPU: Unknown");
        }
    }

    fn cpu_usage(buf: &mut String, sys: &System, c :fn(&str) -> ColoredString) {
        let usage = sys.global_cpu_usage();
        if usage > 99.9 && cfg!(target_os = "windows") {
            let _ = writeln!(buf, "{}: N/A", c("CPU Usage"));
        } else {
            let _ = writeln!(buf, "{}: {:.1}%", c("CPU Usage"), usage);
        }
    }

    fn cpu_temperature(buf: &mut String, c :fn(&str) -> ColoredString) {
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
            let _ = writeln!(buf, "{}: {:.1}°C", c("CPU Temp"), temp);
        }
    }

    fn cpu_cores_and_threads(buf: &mut String, sys: &System, c :fn(&str) -> ColoredString) {
        let _ = writeln!(buf, "{}: {}", c("Cores"), System::physical_core_count().unwrap_or(0));
        let cpus = sys.cpus();
        let _ = writeln!(buf, "{}: {}", c("Threads"), cpus.len());
    }

    fn cpu_arch(buf: &mut String, c :fn(&str) -> ColoredString) {
        let _ = writeln!(buf, "{}: {}", c("Architecture"), std::env::consts::ARCH);
    }
}