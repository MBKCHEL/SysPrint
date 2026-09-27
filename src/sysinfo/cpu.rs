use std::fmt::Write;
use colored::{ColoredString};
use sysinfo::{Components, System};
use crate::sysinfo::combine::DisplayOptions;
use colored::Colorize;

pub fn cpu_info(opts: &DisplayOptions, buf: &mut String, sys: &System, c :fn(&str) -> ColoredString) {
    if !opts.cpu {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", "--- CPU INFO ---".bold().cyan());
    }

    if opts.cpu_name {
        cpu_name(buf, sys, c);
    }
    if opts.compact_mode {
        return;
    }
    if opts.cpu_freq {
        ggz_cpu(buf, sys, c);
    }
    if opts.cpu_usage && !opts.fast_mode {
        cpu_usage(buf, sys, c);
    }
    if opts.cpu_temp && !opts.fast_mode {
        cpu_temperature(buf, c);
    }
    if opts.cpu_cores {
        cpu_cores_and_threads(buf, sys, c);
    }
    if opts.cpu_arch {
        cpu_arch(buf, c);
    }

    fn cpu_name(buf: &mut String, sys: &System, c :fn(&str) -> ColoredString){
        let cpus = sys.cpus();
        if let Some(cpu) = cpus.first() {
            let _ = writeln!(buf, "{}: {}", c("CPU name"), cpu.brand().trim());
        }
        else {
            let _ = writeln!(buf, "CPU: Unknown");
        }
    }

    fn ggz_cpu(buf: &mut String, sys: &System, c :fn(&str) -> ColoredString) {
        let cpus = sys.cpus();
        if let Some(cpu) = cpus.first() {
            let freq_ghz = cpu.frequency() as f64 / 1000.0;
            let _ = writeln!(buf, "{}: {:.2} GHz", c("GHz"), freq_ghz);
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