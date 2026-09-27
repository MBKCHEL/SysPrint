use std::fmt::Write;
use colored::ColoredString;
use sysinfo::{Components, System};
use crate::sysinfo::combine::DisplayOptions;

pub fn cpu_info(opts: &DisplayOptions, buf: &mut String, sys: &System, c :fn(&str) -> ColoredString) {
    if !opts.cpu {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", opts.format_header("--- CPU INFO ---", c));
    }

    if opts.cpu_name {
        cpu_name(buf, sys, c);
    }
    if opts.compact_mode {
        return;
    }
    if opts.cpu_freq {
        ghz_cpu(buf, sys, c);
    }
    if opts.cpu_usage && !opts.fast_mode {
        cpu_usage(opts, buf, sys, c);
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
}

fn cpu_name(buf: &mut String, sys: &System, c: fn(&str) -> ColoredString) {
    let cpus = sys.cpus();
    if let Some(cpu) = cpus.first() {
        let _ = writeln!(buf, "{}: {}", c("CPU name"), cpu.brand().trim());
    } else {
        let _ = writeln!(buf, "{}: Unknown", c("CPU name"));
    }
}

fn ghz_cpu(buf: &mut String, sys: &System, c: fn(&str) -> ColoredString) {
    let cpus = sys.cpus();
    if let Some(cpu) = cpus.first() {
        let freq_ghz = cpu.frequency() as f64 / 1000.0;
        let _ = writeln!(buf, "{}: {:.2} GHz", c("GHz"), freq_ghz);
    }
}

fn cpu_usage(opts: &DisplayOptions, buf: &mut String, sys: &System, c: fn(&str) -> ColoredString) {
    let usage = sys.global_cpu_usage();
    let bar = if opts.progress_bars {
        format!("{} ", crate::sysinfo::combine::make_bar(usage as f64, 10, c))
    } else {
        String::new()
    };
    let _ = writeln!(buf, "{}: {}{:.1}%", c("CPU Usage"), bar, usage);
}

pub fn get_cpu_temp() -> Option<f32> {
    let components = Components::new_with_refreshed_list();
    components.iter().find_map(|comp| {
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
    })
}

fn cpu_temperature(buf: &mut String, c: fn(&str) -> ColoredString) {
    if let Some(temp) = get_cpu_temp() {
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