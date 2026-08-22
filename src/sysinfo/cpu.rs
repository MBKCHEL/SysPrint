use std::fmt::Write;
use colored::Colorize;
use sysinfo::{Components, System};
use crate::sysinfo::combine::DisplayOptions;

pub fn cpu_info(opts: &DisplayOptions, buf: &mut String, sys: &System) {
    if !opts.cpu {
        return;
    }

    let _ = writeln!(buf, "{}", "--- CPU INFO ---".bold().cyan());

    ggz_and_name_cpu(buf, sys);
    cpu_usage(buf, sys);
    cpu_temperature(buf);
    cpu_cores_and_threads(buf, sys);
    cpu_arch(buf);

    fn ggz_and_name_cpu(buf: &mut String, sys: &System) {
        let cpus = sys.cpus();
        if let Some(cpu) = cpus.first() {
            let _ = writeln!(buf, "{}: {}", "CPU name".bold(), cpu.brand().trim());
            let freq_ghz = cpu.frequency() as f64 / 1000.0;
            let _ = writeln!(buf, "{}: {:.2} GHz", "GHz".bold(), freq_ghz);
        } else {
            let _ = writeln!(buf, "CPU: Unknown");
        }
    }

    fn cpu_usage(buf: &mut String, sys: &System) {
        let usage = sys.global_cpu_usage();
        if usage > 99.9 && cfg!(target_os = "windows") {
            let _ = writeln!(buf, "{}: N/A", "CPU Usage".bold());
        } else {
            let _ = writeln!(buf, "{}: {:.1}%", "CPU Usage".bold(), usage);
        }
    }

    fn cpu_temperature(buf: &mut String) {
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
            let _ = writeln!(buf, "{}: {:.1}°C", "CPU Temp".bold(), temp);
        }
    }

    fn cpu_cores_and_threads(buf: &mut String, sys: &System) {
        let _ = writeln!(
            buf,
            "{}: {}",
            "Cores".bold(),
            System::physical_core_count().unwrap_or(0)
        );
        let cpus = sys.cpus();
        let _ = writeln!(buf, "{}: {}", "Threads".bold(), cpus.len());
    }

    fn cpu_arch(buf: &mut String) {
        let _ = writeln!(buf, "{}: {}", "Architecture".bold(), std::env::consts::ARCH);
    }
}