use crate::sysinfo::combine::DisplayOptions;
use colored::{ColoredString, Colorize};
use pci_ids::FromId;
use std::fmt::Write;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_gpu_info(opts: &DisplayOptions, buf: &mut String, c: fn(&str) -> ColoredString) {
    if !opts.gpu {
        return;
    }

    let _ = writeln!(buf, "{}", "--- GPU INFO ---".bold().cyan());

    if get_nvidia_info(buf, c) {
        return;
    }

    #[cfg(target_os = "macos")]
    if get_macos_gpu_info(buf, c) {
        return;
    }

    #[cfg(target_os = "linux")]
    if get_linux_sysfs_gpu(buf, c) {
        return;
    }

    #[cfg(windows)]
    if get_windows_gpu_info(buf, c) {
        return;
    }

    #[cfg(target_os = "freebsd")]
    if get_freebsd_gpu_info(buf, c) {
        return;
    }

    #[cfg(target_os = "openbsd")]
    if get_openbsd_gpu_info(buf, c) {
        return;
    }

    #[cfg(target_os = "netbsd")]
    if get_netbsd_gpu_info(buf, c) {
        return;
    }

    let name = clean_gpu_name(&get_generic_gpu_name());
    let _ = writeln!(buf, "{}: {}", c("GPU"), name);
}

fn clean_gpu_name(raw: &str) -> String {
    let mut name = raw.to_string();
    
    let prefixes = [
        "Advanced Micro Devices, Inc. [AMD/ATI]",
        "Advanced Micro Devices, Inc.",
        "NVIDIA Corporation",
        "Intel Corporation",
    ];

    for prefix in prefixes {
        if name.starts_with(prefix) {
            name = name.replacen(prefix, "", 1).trim().to_string();
            break;
        }
    }

    if let Some(pos) = name.rfind("(rev ") {
        name = name[..pos].trim().to_string();
    }

    // 3. Убираем внешние квадратные скобки, если они остались
    if name.starts_with('[') && name.ends_with(']') {
        name = name[1..name.len() - 1].trim().to_string();
    }

    if name.is_empty() {
        "Unknown GPU".to_string()
    } else {
        name
    }
}

fn get_nvidia_info(buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=gpu_name,memory.total,memory.used,temperature.gpu",
            "--format=csv,noheader,nounits",
        ])
        .output();

    let output = match output {
        Ok(out) => out,
        Err(_) => return false,
    };

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        return false;
    }

    let parts: Vec<&str> = stdout.split(',').map(|s| s.trim()).collect();
    if parts.len() < 4 {
        return false;
    }

    let name = parts[0];
    let mem_total: f64 = parts[1].parse().unwrap_or(0.0) / 1024.0;
    let mem_used: f64 = parts[2].parse().unwrap_or(0.0) / 1024.0;
    let temp = parts[3];

    let _ = writeln!(buf, "{}: {}", c("GPU"), name);
    let _ = writeln!(buf, "{}: {:.2} GB / {:.2} GB", c("VRAM"), mem_used, mem_total);
    let _ = writeln!(buf, "{}: {}°C", c("GPU Temp"), temp);

    true
}

#[cfg(target_os = "macos")]
fn get_macos_gpu_info(buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let output = match Command::new("system_profiler").arg("SPDisplaysDataType").output() {
        Ok(out) => out,
        Err(_) => return false,
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let mut gpu_name = String::new();
    let mut vram = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Chipset Model:") {
            if let Some(pos) = trimmed.find(':') {
                gpu_name = trimmed[pos + 1..].trim().to_string();
            }
        } else if trimmed.starts_with("VRAM (Total):") || trimmed.starts_with("VRAM (Dynamic):") {
            if let Some(pos) = trimmed.find(':') {
                vram = trimmed[pos + 1..].trim().to_string();
            }
        }
    }

    if gpu_name.is_empty() {
        return false;
    }

    let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(&gpu_name));
    if !vram.is_empty() {
        let _ = writeln!(buf, "{}: {}", c("VRAM"), vram);
    }

    true
}

#[cfg(target_os = "linux")]
fn get_linux_sysfs_gpu(buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let drm_path = Path::new("/sys/class/drm");
    if !drm_path.exists() {
        return false;
    }

    let Ok(entries) = fs::read_dir(drm_path) else {
        return false;
    };

    for entry in entries.flatten() {
        let name_str = entry.file_name().to_string_lossy().into_owned();

        if !name_str.starts_with("card") || name_str.contains('-') {
            continue;
        }

        let device_path = entry.path().join("device");
        if !device_path.exists() {
            continue;
        }

        let mut gpu_name = String::new();

        if let Ok(target) = fs::read_link(&device_path) {
            if let Some(pci_slot) = target.file_name().and_then(|s| s.to_str()) {
                if let Ok(output) = Command::new("lspci").args(["-s", pci_slot]).output() {
                    let text = String::from_utf8_lossy(&output.stdout);
                    if let Some(line) = text.lines().next() {
                        if let Some(pos) = line.find(':') {
                            let raw = line[pos + 1..].trim();
                            gpu_name = raw.split(':').last().unwrap_or(raw).trim().to_string();
                        }
                    }
                }
            }
        }

        if gpu_name.is_empty() {
            let vendor_hex = fs::read_to_string(device_path.join("vendor")).unwrap_or_default();
            let device_hex = fs::read_to_string(device_path.join("device")).unwrap_or_default();

            let vendor_id = u16::from_str_radix(vendor_hex.trim().trim_start_matches("0x"), 16).unwrap_or(0);
            let device_id = u16::from_str_radix(device_hex.trim().trim_start_matches("0x"), 16).unwrap_or(0);

            if let Some(vendor) = pci_ids::Vendor::from_id(vendor_id) {
                if let Some(device) = vendor.devices().find(|d| d.id() == device_id) {
                    gpu_name = device.name().to_string();
                } else {
                    gpu_name = format!("{} Graphics", vendor.name());
                }
            }
        }

        gpu_name = clean_gpu_name(&gpu_name);

        let _ = writeln!(buf, "{}: {}", c("GPU"), gpu_name);

        // VRAM
        let vram_used_path = device_path.join("mem_info_vram_used");
        let vram_total_path = device_path.join("mem_info_vram_total");

        if vram_used_path.exists() && vram_total_path.exists() {
            let used_bytes: f64 = fs::read_to_string(vram_used_path)
                .unwrap_or_default()
                .trim()
                .parse()
                .unwrap_or(0.0);
            let total_bytes: f64 = fs::read_to_string(vram_total_path)
                .unwrap_or_default()
                .trim()
                .parse()
                .unwrap_or(0.0);

            if total_bytes > 0.0 {
                let used_gb = used_bytes / 1024.0 / 1024.0 / 1024.0;
                let total_gb = total_bytes / 1024.0 / 1024.0 / 1024.0;
                let _ = writeln!(buf, "{}: {:.2} GB / {:.2} GB", c("VRAM"), used_gb, total_gb);
            }
        }

        // Temp
        let hwmon_dir = device_path.join("hwmon");
        if let Ok(hwmon_entries) = fs::read_dir(hwmon_dir) {
            for hwmon in hwmon_entries.flatten() {
                let temp_path = hwmon.path().join("temp1_input");
                if temp_path.exists() {
                    if let Ok(temp_raw) = fs::read_to_string(temp_path) {
                        if let Ok(temp_mc) = temp_raw.trim().parse::<f64>() {
                            let _ = writeln!(buf, "{}: {:.0}°C", c("GPU Temp"), temp_mc / 1000.0);
                            break;
                        }
                    }
                }
            }
        }

        return true;
    }

    false
}

#[cfg(windows)]
fn get_windows_gpu_info(buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let script = "Get-CimInstance Win32_VideoController | Select-Object Name, AdapterRAM | ConvertTo-Json";
    let output = match Command::new("powershell").args(["-Command", script]).output() {
        Ok(out) => out,
        Err(_) => return false,
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return false;
    }

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
        let obj = if json.is_array() { &json[0] } else { &json };
        let raw_name = obj["Name"].as_str().unwrap_or("Unknown GPU");
        let name = clean_gpu_name(raw_name);
        let vram_bytes = obj["AdapterRAM"].as_f64().unwrap_or(0.0);
        let vram_gb = vram_bytes / 1024.0 / 1024.0 / 1024.0;

        let _ = writeln!(buf, "{}: {}", c("GPU"), name);
        if vram_gb > 0.0 {
            let _ = writeln!(buf, "{}: {:.2} GB", c("VRAM"), vram_gb);
        }
        return true;
    }

    false
}

#[cfg(target_os = "freebsd")]
fn get_freebsd_gpu_info(buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let output = match Command::new("sh").arg("-c").arg("pciconf -lv | grep -B 4 -i 'class=0x03'").output() {
        Ok(out) => out,
        Err(_) => return false,
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let mut name = String::new();

    for line in text.lines() {
        if line.trim().starts_with("device") {
            if let Some(pos) = line.find("='") {
                name = line[pos + 2..].trim_matches('\'').to_string();
                break;
            }
        }
    }

    if name.is_empty() {
        return false;
    }

    let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(&name));

    if let Ok(sysctl_out) = Command::new("sysctl").arg("-n").arg("dev.amdtemp.0.core0").output() {
        let temp_str = String::from_utf8_lossy(&sysctl_out.stdout).trim().to_string();
        if !temp_str.is_empty() {
            let _ = writeln!(buf, "{}: {}", c("GPU Temp"), temp_str);
        }
    }

    true
}

#[cfg(target_os = "openbsd")]
fn get_openbsd_gpu_info(buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let output = match Command::new("sh").arg("-c").arg("pcidump -v | grep -i 'vga'").output() {
        Ok(out) => out,
        Err(_) => return false,
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let line = match text.lines().next() {
        Some(l) => l,
        None => return false,
    };

    let name = if let Some(pos) = line.rfind(':') {
        line[pos + 1..].trim().to_string()
    } else {
        line.trim().to_string()
    };

    if name.is_empty() {
        return false;
    }

    let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(&name));
    true
}

#[cfg(target_os = "netbsd")]
fn get_netbsd_gpu_info(buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let output = match Command::new("sh").arg("-c").arg("pcictl pci0 list | grep -i 'display'").output() {
        Ok(out) => out,
        Err(_) => return false,
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let line = match text.lines().next() {
        Some(l) => l,
        None => return false,
    };

    let pos = match line.find(':') {
        Some(p) => p,
        None => return false,
    };

    let name = line[pos + 1..].trim().to_string();
    if name.is_empty() {
        return false;
    }

    let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(&name));

    if let Ok(env_out) = Command::new("envstat").args(["-s", "amdgpu:temperature"]).output() {
        let env_text = String::from_utf8_lossy(&env_out.stdout);
        if let Some(temp_line) = env_text.lines().find(|l| l.contains("degC")) {
            let parts: Vec<&str> = temp_line.split_whitespace().collect();
            if parts.len() >= 2 {
                let _ = writeln!(buf, "{}: {}°C", c("GPU Temp"), parts[1]);
            }
        }
    }

    true
}

fn get_generic_gpu_name() -> String {
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("system_profiler").arg("SPDisplaysDataType").output() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Chipset Model:") {
                    if let Some(pos) = trimmed.find(':') {
                        return trimmed[pos + 1..].trim().to_string();
                    }
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = Command::new("sh").args(["-c", "lspci | grep -Ei 'vga|3d|display'"]).output() {
            let text = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = text.lines().next() {
                if let Some(pos) = line.find(':') {
                    let raw_name = line[pos + 1..].trim();
                    return raw_name.split(':').last().unwrap_or(raw_name).trim().to_string();
                }
            }
        }
    }

    #[cfg(windows)]
    {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !text.is_empty() {
                return text.lines().next().unwrap_or("Unknown GPU").to_string();
            }
        }
    }

    "Unknown GPU".to_string()
}