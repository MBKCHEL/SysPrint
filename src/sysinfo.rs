//! Helper functions for retrieving system information.

mod combine;

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

        let sys = System::new_all();


        // Collect all system information into a vector of strings
        let mut lines: Vec<String> = Vec::new();

        system_info(&opts, &mut lines);

        // --- SYSTEM INFO ---
        fn system_info(opts: &DisplayOptions, lines: &mut Vec<String>){

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

        cpu_info(&opts, &mut lines, &sys);

        // --- CPU INFO ---
        fn cpu_info(opts: &DisplayOptions, mut lines: &mut Vec<String>, sys: &System){

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

        }


        memory_info(&opts, &mut lines, &sys);

        fn memory_info(opts: &DisplayOptions, lines: &mut Vec<String>, sys: &System) {
            // --- MEMORY INFO ---
            opts.memory;
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

        disk_info(&opts, &mut lines);

        fn disk_info(opts: &DisplayOptions, lines: &mut Vec<String>) {
            // --- DISKS INFO ---
            opts.disks;
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


        // give link functions other_info for variables lines
        other_info(&opts, &mut lines);


        // Functions other_info
        fn other_info(opts: &DisplayOptions, mut lines: &mut Vec<String>) {
            // --- Other Info ---
            opts.other;
            lines.push(format!("{}", "--- Other Info ---".bold().cyan()));

            // give link functions de_wm_check for variables lines
            de_wm_check(&mut lines);

            // Functions de_wm_check
            fn de_wm_check(lines: &mut Vec<String>){
                // Desktop Environment / Window Manager
                let desktop = env::var("XDG_CURRENT_DESKTOP")
                    .or_else(|_| env::var("DESKTOP_SESSION"))
                    .unwrap_or_else(|_| "Unknown".to_string());
                lines.push(format!("{}: {}", "DE/WM".bold(), desktop));
            }

            // give link functions battery for variables lines
            battery(&mut lines);

            // Functions battery
            fn battery(lines: &mut Vec<String>){
                let mut battery = "N/A (Desktop)".to_string();

                // Linux battery
                #[cfg(target_os = "linux")]
                {


                    if let (Ok(cap), Ok(stat)) = (
                        fs::read_to_string("/sys/class/power_supply/BAT0/capacity"),
                        fs::read_to_string("/sys/class/power_supply/BAT0/status"),
                    ) {
                        battery = format!("{}% [{}]", cap.trim(), stat.trim());
                    } else if let (Ok(cap), Ok(stat)) = (
                        fs::read_to_string("/sys/class/power_supply/BAT1/capacity"),
                        fs::read_to_string("/sys/class/power_supply/BAT1/status"),
                    ) {
                        battery = format!("{}% [{}]", cap.trim(), stat.trim());
                    }
                }

                // FreeBSD battery
                #[cfg(target_os = "freebsd")]
                {
                    let output = Command::new("sysctl")
                        .arg("-n")
                        .arg("hw.acpi.battery.life")
                        .output();

                    if let Ok(out) = output {
                        let cap = String::from_utf8_lossy(&out.stdout).trim().to_string();
                        if !cap.is_empty() {
                            battery = format!("{}%", cap);
                        }
                    }
                }
                // OpenBSD battery
                #[cfg(target_os = "openbsd")]
                {
                    // apm -l
                    let output = Command::new("apm").arg("-l").output();

                    if let Ok(out) = output {
                        let cap = String::from_utf8_lossy(&out.stdout).trim().to_string();
                        // 255 в apm означает "батарея не найдена / ПК от сети"
                        if !cap.is_empty() && cap != "255" {
                            battery = format!("{}%", cap);
                        }
                    }
                }
                // NetBSD battery
                #[cfg(target_os = "netbsd")]
                {
                    //envstat
                    let output = Command::new("envstat").args(["-s", "bat0:charge"]).output();

                    if let Ok(out) = output {
                        let text = String::from_utf8_lossy(&out.stdout);
                        if let Some(line) = text.lines().find(|l| l.contains("%")) {
                            if let Some(val) = line.split('(').next() {
                                let cleaned = val.replace("charge:", "").replace("%", "");
                                let trimmed = cleaned.trim();
                                if !trimmed.is_empty() {
                                    battery = format!("{}%", trimmed);
                                }
                            }
                        }
                    }
                }
                // Windows battery
                #[cfg(windows)]
                {
                    let output = Command::new("wmic")
                        .args(["path", "Win32_Battery", "get", "EstimatedChargeRemaining"])
                        .output();

                    if let Ok(out) = output {
                        let text = String::from_utf8_lossy(&out.stdout);
                        if let Some(cap) = text.lines().nth(1) {
                            let trimmed = cap.trim();
                            if !trimmed.is_empty() {
                                battery = format!("{}%", trimmed);
                            }
                        }
                    }
                }

                lines.push(format!("{}: {}", "Battery".bold(), battery));
            }
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


    let mut name = String::new();

    // Windows GPU
    #[cfg(windows)]
    {
        let output = Command::new("powershell").args(["-Command", "Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name", ])
            .output();
        if let Ok(out) = output {
            let gpu = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !gpu.is_empty() {
                name = gpu.lines().next().unwrap_or("Unknown GPU").to_string();
            }
        }
    }

    // Linux GPU
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

    // FreeBSD GPU
    #[cfg(target_os = "freebsd")]
    {
        let output = Command::new("sh")
            .arg("-c")
            .arg("pciconf -lv | grep -B 4 -i 'class=0x03'")
            .output();

        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.trim().starts_with("device") {
                    if let Some(pos) = line.find("='") {
                        name = line[pos + 2..].trim_matches('\'').to_string();
                        break;
                    }
                }
            }
        }
    }

    // OpenBSD GPU
    #[cfg(target_os = "openbsd")]
    {
        //pcidump
        let output = Command::new("sh")
            .arg("-c")
            .arg("pcidump -v | grep -i 'vga'")
            .output();

        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            if let Some(line) = text.lines().next() {
                if let Some(pos) = line.rfind(':') {
                    name = line[pos + 1..].trim().to_string();
                } else {
                    name = line.trim().to_string();
                }
            }
        }
    }

    // NetBSD GPU
    #[cfg(target_os = "netbsd")]
    {
        // pcictl
        let output = Command::new("sh")
            .arg("-c")
            .arg("pcictl pci0 list | grep -i 'display'")
            .output();

        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            if let Some(line) = text.lines().next() {
                if let Some(pos) = line.find(':') {
                    name = line[pos + 1..].trim().to_string();
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

