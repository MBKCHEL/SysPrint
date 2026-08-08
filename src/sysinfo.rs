//! Helper functions for retrieving system information.
use std::env;
use std::fs;
use std::process::Command;

use colored::Colorize;
use sysinfo::{Components, Disks, System};

/// Which sections of the report should be collected.
#[derive(Clone, Copy)]
pub struct DisplayOptions {
    pub system: bool,
    pub cpu: bool,
    pub memory: bool,
    pub disks: bool,
    pub other: bool,
    pub gpu: bool,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            system: true,
            cpu: true,
            memory: true,
            disks: true,
            other: true,
            gpu: true,
        }
    }
}

pub struct SystemInfo {
    pub lines: Vec<String>,
    pub gpu: Vec<String>,
}

impl SystemInfo {
    /// Collect all system information into a renderable form.
    pub fn collect(opts: DisplayOptions) -> Self {
        // For fucking Windows CMD
        #[cfg(windows)]
        {
            use colored::control::set_virtual_terminal;
            let _ = set_virtual_terminal(true);
        }

        let mut sys = System::new_all();
        sys.refresh_all();
        sys.refresh_cpu_all();

        // Collect all system information into a vector of strings
        let mut lines: Vec<String> = Vec::new();

        // --- SYSTEM INFO ---
        if opts.system {
            lines.push(format!("{}", "--- System INFO ---".bold().cyan()));
            lines.push(format!(
                "{}: {}",
                "OS".bold(),
                System::name().unwrap_or_default()
            ));
            lines.push(format!(
                "{}: {}",
                "OS Version".bold(),
                System::os_version().unwrap_or_default()
            ));
            lines.push(format!(
                "{}: {}",
                "Host".bold(),
                System::host_name().unwrap_or_default()
            ));
        }

        // --- CPU INFO ---
        if opts.cpu {
            lines.push(format!("{}", "--- CPU INFO ---".bold().cyan()));

            // GHz and name CPU
            let cpus = sys.cpus();
            if let Some(cpu) = cpus.first() {
                lines.push(format!("{}: {}", "CPU".bold(), cpu.brand().trim()));
                let freq_ghz = cpu.frequency() as f64 / 1000.0;
                lines.push(format!("{}: {:.2} GHz", "Freq".bold(), freq_ghz));
            } else {
                lines.push("CPU: Unknown".to_string());
            }

            // CPU Usage
            lines.push(format!(
                "{}: {:.1}%",
                "CPU Usage".bold(),
                sys.global_cpu_usage()
            ));

            // CPU Temperature
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

            // Processor Cores & Threads
            lines.push(format!(
                "{}: {}",
                "Cores".bold(),
                System::physical_core_count().unwrap_or(0)
            ));
            lines.push(format!("{}: {}", "Threads".bold(), cpus.len()));
        }

        // --- MEMORY INFO ---
        if opts.memory {
            lines.push(format!("{}", "--- Memory INFO ---".bold().cyan()));
            let total_ram = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
            let used_ram = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
            lines.push(format!(
                "{}: {:.2} GB / {:.2} GB",
                "RAM".bold(),
                used_ram,
                total_ram
            ));
        }

        // --- DISKS INFO ---
        if opts.disks {
            lines.push(format!("{}", "--- Disks INFO ---".bold().cyan()));
            let disks = Disks::new_with_refreshed_list();
            for disk in &disks {
                let total_gb = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
                let available_gb = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
                let used_gb = total_gb - available_gb;

                let mount_point = disk.mount_point().to_string_lossy();
                lines.push(format!(
                    "{}: {:.2} GB / {:.2} GB ({})",
                    mount_point.bold(),
                    used_gb,
                    total_gb,
                    disk.file_system().to_string_lossy()
                ));
            }
        }

        // --- Other Info ---
        if opts.other {
            lines.push(format!("{}", "--- Other Info ---".bold().cyan()));

            // Desktop Environment / Window Manager
            let desktop = env::var("XDG_CURRENT_DESKTOP")
                .or_else(|_| env::var("DESKTOP_SESSION"))
                .unwrap_or_else(|_| "Unknown".to_string());
            lines.push(format!("{}: {}", "DE/WM".bold(), desktop));

            // Battery status (checks /sys/class/power_supply/BAT0 or BAT1)
            let battery = match (
                fs::read_to_string("/sys/class/power_supply/BAT0/capacity"),
                fs::read_to_string("/sys/class/power_supply/BAT0/status"),
            ) {
                (Ok(cap), Ok(stat)) => format!("{}% [{}]", cap.trim(), stat.trim()),
                _ => match (
                    fs::read_to_string("/sys/class/power_supply/BAT1/capacity"),
                    fs::read_to_string("/sys/class/power_supply/BAT1/status"),
                ) {
                    (Ok(cap), Ok(stat)) => format!("{}% [{}]", cap.trim(), stat.trim()),
                    _ => "N/A (Desktop)".to_string(),
                },
            };
            lines.push(format!("{}: {}", "Battery".bold(), battery));
        }

        let gpu = if opts.gpu {
            get_gpu_info()
        } else {
            Vec::new()
        };

        Self { lines, gpu }
    }

    /// Returns all lines including the GPU section (if present).
    pub fn all_lines(&self) -> Vec<String> {
        let mut all = self.lines.clone();
        if !self.gpu.is_empty() {
            all.push(format!("{}", "--- GPU INFO ---".bold().cyan()));
            all.extend(self.gpu.iter().cloned());
        }
        all
    }
}

pub fn get_gpu_info() -> Vec<String> {
    let mut gpu_lines = Vec::new();

    // Temp And Vram
    let nvidia_output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=gpu_name,memory.total,memory.used,temperature.gpu",
            "--format=csv,noheader,nounits",
        ])
        .output();

    if let Ok(out) = nvidia_output {
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !stdout.is_empty() {
            let parts: Vec<&str> = stdout.split(',').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                let name = parts[0];
                let mem_total: f64 = parts[1].parse().unwrap_or(0.0) / 1024.0;
                let mem_used: f64 = parts[2].parse().unwrap_or(0.0) / 1024.0;
                let temp = parts[3];

                gpu_lines.push(format!("{}: {}", "GPU".bold(), name));
                gpu_lines.push(format!(
                    "{}: {:.2} GB / {:.2} GB",
                    "VRAM".bold(),
                    mem_used,
                    mem_total
                ));
                gpu_lines.push(format!("{}: {}°C", "GPU Temp".bold(), temp));

                return gpu_lines;
            }
        }
    }

    // If not nvidia we give name.
    let mut name = String::new();

    #[cfg(windows)]
    {
        let output = Command::new("powershell")
            .args([
                "-Command",
                "Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name",
            ])
            .output();

        if let Ok(out) = output {
            let gpu = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !gpu.is_empty() {
                name = gpu.lines().next().unwrap_or("Unknown GPU").to_string();
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("sh")
            .arg("-c")
            .arg("lspci | grep -Ei 'vga|3d|display'")
            .output();

        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            if let Some(line) = text.lines().next() {
                if let Some(pos) = line.find(':') {
                    let raw_name = line[pos + 1..].trim();
                    name = raw_name
                        .split(':')
                        .last()
                        .unwrap_or(raw_name)
                        .trim()
                        .to_string();
                }
            }
        }
    }

    if name.is_empty() {
        name = "Unknown GPU".to_string();
    }

    gpu_lines.push(format!("{}: {}", "GPU".bold(), name));
    gpu_lines
}