use colored::Colorize;
use std::process::Command;
use crate::sysinfo::combine::DisplayOptions;

pub fn get_gpu_info(opts: &DisplayOptions) -> Vec<String> {
    let mut name = String::new();
    let mut gpu_lines = Vec::new();
    if opts.gpu {
        gpu_lines.push(format!("{}", "--- GPU INFO ---".bold().cyan()));

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
    }
        if name.is_empty() {
            name = "Unknown GPU".to_string();
        }

        gpu_lines.push(format!("{}: {}", "GPU".bold(), name));
        gpu_lines

}
