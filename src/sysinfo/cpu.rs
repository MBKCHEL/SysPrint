use colored::Colorize;
use std::thread;
use sysinfo::{Components, System, MINIMUM_CPU_UPDATE_INTERVAL};
use crate::sysinfo::combine::DisplayOptions;

// --- CPU INFO ---
pub fn cpu_info(opts: &DisplayOptions, mut lines: &mut Vec<String>, sys: &System){

        let mut sys = System::new_all();


    sys.refresh_cpu_usage();


    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);


    sys.refresh_cpu_usage();




    opts.cpu;
    lines.push(format!("{}", "--- CPU INFO ---".bold().cyan()));

    ggz_and_name_cpu(&mut lines, &sys);
    fn ggz_and_name_cpu(lines: &mut Vec<String>, sys: &System){
        // GHz and name CPU
        let cpus = sys.cpus();
        if let Some(cpu) = cpus.first() {
            lines.push(format!("{}: {}", "CPU name".bold(), cpu.brand().trim()));
            let freq_ghz = cpu.frequency() as f64 / 1000.0;
            lines.push(format!("{}: {:.2} GHz", "GHz".bold(), freq_ghz));
        } else {
            lines.push("CPU: Unknown".to_string());
        }
    }

    cpu_usage(&mut lines, &sys);

    fn cpu_usage(lines: &mut Vec<String>, sys: &System){
        // CPU Usage
        lines.push(format!(
            "{}: {:.1}%",
            "CPU Usage".bold(),
            sys.global_cpu_usage()
        ));
    }

    cpu_temperature(&mut lines);

    // CPU Temperature
    fn cpu_temperature(lines: &mut Vec<String>){

        let components = Components::new_with_refreshed_list();
        let cpu_temp = components.iter().find_map(|comp| {
            let label = comp.label().to_lowercase();
            if label.contains("cpu")
                || label.contains("core")
                || label.contains("package")
                || label.contains("k10temp")
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

    cpu_cores_and_threads(&mut lines, &sys);

    // cpu_cores_and_threads
    fn cpu_cores_and_threads(lines: &mut Vec<String>, sys: &System) {
        lines.push(format!(
            "{}: {}",
            "Cores".bold(),
            System::physical_core_count().unwrap_or(0)
        ));
        let cpus = sys.cpus();
        lines.push(format!("{}: {}", "Threads".bold(), cpus.len()));
    }

    cpu_arch(lines);
    // CPU Architecture
    fn cpu_arch(lines: &mut Vec<String>) {
        lines.push(format!(
            "{}: {}",
            "Architecture".bold(),
            std::env::consts::ARCH,
        ));
    }
}