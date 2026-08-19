use colored::Colorize;
use std::thread;
use sysinfo::{Components, System, MINIMUM_CPU_UPDATE_INTERVAL};
use crate::sysinfo::combine::DisplayOptions;

#[cfg(target_os = "windows")]
use std::process::Command;

// --- CPU INFO ---
pub fn cpu_info(opts: &DisplayOptions, mut lines: &mut Vec<String>, _sys: &System) {

    let mut sys = System::new_all();

    sys.refresh_cpu_usage();

    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_usage();

    opts.cpu;
    lines.push(format!("{}", "--- CPU INFO ---".bold().cyan()));

    ggz_and_name_cpu(&mut lines, &sys);
    fn ggz_and_name_cpu(lines: &mut Vec<String>, sys: &System) {
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

    fn cpu_usage(lines: &mut Vec<String>, sys: &System) {
        // CPU Usage
        lines.push(format!(
            "{}: {:.1}%",
            "CPU Usage".bold(),
            sys.global_cpu_usage()
        ));
    }

    cpu_temperature(&mut lines);

    // CPU Temperature
    fn cpu_temperature(lines: &mut Vec<String>) {

        let components = Components::new_with_refreshed_list();
        let mut cpu_temp = components.iter().find_map(|comp| {
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

        #[cfg(target_os = "windows")]
        if cpu_temp.is_none() {
            cpu_temp = get_windows_cpu_temp();
        }

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

#[cfg(target_os = "windows")]
fn get_windows_cpu_temp() -> Option<f32> {
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "(Get-CimInstance -Namespace root/WMI -ClassName MSAcpi_ThermalZoneTemperature).CurrentTemperature",
        ])
        .output()
        .ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(raw_temp) = stdout.trim().parse::<f32>() {
            let celsius = (raw_temp / 10.0) - 273.15;
            if celsius > 0.0 && celsius < 120.0 {
                return Some(celsius);
            }
        }
    }
    None
}