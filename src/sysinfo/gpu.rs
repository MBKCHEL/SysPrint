use colored::Colorize;
use std::fs;
use std::path::Path;
use std::process::Command;
use crate::sysinfo::combine::DisplayOptions;

pub fn get_gpu_info(opts: &DisplayOptions) -> Vec<String> {
    let mut gpu_lines = Vec::new();
    if !opts.gpu {
        return gpu_lines;
    }

    gpu_lines.push(format!("{}", "--- GPU INFO ---".bold().cyan()));

    if let Some(nvidia_lines) = get_nvidia_info() {
        gpu_lines.extend(nvidia_lines);
        return gpu_lines;
    }

    #[cfg(target_os = "macos")]
    if let Some(mac_lines) = get_macos_gpu_info() {
        gpu_lines.extend(mac_lines);
        return gpu_lines;
    }

    #[cfg(target_os = "linux")]
    if let Some(sysfs_lines) = get_linux_sysfs_gpu() {
        gpu_lines.extend(sysfs_lines);
        return gpu_lines;
    }

    #[cfg(windows)]
    if let Some(win_lines) = get_windows_gpu_info() {
        gpu_lines.extend(win_lines);
        return gpu_lines;
    }

    #[cfg(target_os = "freebsd")]
    if let Some(bsd_lines) = get_freebsd_gpu_info() {
        gpu_lines.extend(bsd_lines);
        return gpu_lines;
    }

    #[cfg(target_os = "openbsd")]
    if let Some(bsd_lines) = get_openbsd_gpu_info() {
        gpu_lines.extend(bsd_lines);
        return gpu_lines;
    }

    #[cfg(target_os = "netbsd")]
    if let Some(bsd_lines) = get_netbsd_gpu_info() {
        gpu_lines.extend(bsd_lines);
        return gpu_lines;
    }

    let name = get_generic_gpu_name();
    gpu_lines.push(format!("{}: {}", "GPU".bold(), name));
    gpu_lines
}

fn get_nvidia_info() -> Option<Vec<String>> {
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=gpu_name,memory.total,memory.used,temperature.gpu",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        return None;
    }

    let parts: Vec<&str> = stdout.split(',').map(|s| s.trim()).collect();
    if parts.len() < 4 {
        return None;
    }

    let name = parts[0];
    let mem_total: f64 = parts[1].parse().unwrap_or(0.0) / 1024.0;
    let mem_used: f64 = parts[2].parse().unwrap_or(0.0) / 1024.0;
    let temp = parts[3];

    Some(vec![
        format!("{}: {}", "GPU".bold(), name),
        format!("{}: {:.2} GB / {:.2} GB", "VRAM".bold(), mem_used, mem_total),
        format!("{}: {}°C", "GPU Temp".bold(), temp),
    ])
}

#[cfg(target_os = "macos")]
fn get_macos_gpu_info() -> Option<Vec<String>> {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
        .ok()?;

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
        return None;
    }

    let mut lines = vec![format!("{}: {}", "GPU".bold(), gpu_name)];
    if !vram.is_empty() {
        lines.push(format!("{}: {}", "VRAM".bold(), vram));
    }

    Some(lines)
}

#[cfg(target_os = "linux")]
fn get_linux_sysfs_gpu() -> Option<Vec<String>> {
    let base_path = Path::new("/sys/class/drm/card0/device");
    if !base_path.exists() {
        return None;
    }

    let mut lines = Vec::new();
    let name = get_generic_gpu_name();
    lines.push(format!("{}: {}", "GPU".bold(), name));

    let vram_used_path = base_path.join("mem_info_vram_used");
    let vram_total_path = base_path.join("mem_info_vram_total");

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
            lines.push(format!("{}: {:.2} GB / {:.2} GB", "VRAM".bold(), used_gb, total_gb));
        }
    }

    let hwmon_dir = base_path.join("hwmon");
    if let Ok(entries) = fs::read_dir(hwmon_dir) {
        for entry in entries.flatten() {
            let temp_path = entry.path().join("temp1_input");
            if temp_path.exists() {
                if let Ok(temp_raw) = fs::read_to_string(temp_path) {
                    if let Ok(temp_mc) = temp_raw.trim().parse::<f64>() {
                        lines.push(format!("{}: {:.0}°C", "GPU Temp".bold(), temp_mc / 1000.0));
                        break;
                    }
                }
            }
        }
    }

    if lines.len() > 1 {
        Some(lines)
    } else {
        None
    }
}

#[cfg(windows)]
fn get_windows_gpu_info() -> Option<Vec<String>> {
    let script = "Get-CimInstance Win32_VideoController | Select-Object Name, AdapterRAM | ConvertTo-Json";
    let output = Command::new("powershell")
        .args(["-Command", script])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return None;
    }

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
        let obj = if json.is_array() { &json[0] } else { &json };
        let name = obj["Name"].as_str().unwrap_or("Unknown GPU");
        let vram_bytes = obj["AdapterRAM"].as_f64().unwrap_or(0.0);
        let vram_gb = vram_bytes / 1024.0 / 1024.0 / 1024.0;

        let mut lines = vec![format!("{}: {}", "GPU".bold(), name)];
        if vram_gb > 0.0 {
            lines.push(format!("{}: {:.2} GB", "VRAM".bold(), vram_gb));
        }
        return Some(lines);
    }

    None
}

#[cfg(target_os = "freebsd")]
fn get_freebsd_gpu_info() -> Option<Vec<String>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pciconf -lv | grep -B 4 -i 'class=0x03'")
        .output()
        .ok()?;

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
        return None;
    }

    let mut lines = vec![format!("{}: {}", "GPU".bold(), name)];

    if let Ok(sysctl_out) = Command::new("sysctl").arg("-n").arg("dev.amdtemp.0.core0").output() {
        let temp_str = String::from_utf8_lossy(&sysctl_out.stdout).trim().to_string();
        if !temp_str.is_empty() {
            lines.push(format!("{}: {}", "GPU Temp".bold(), temp_str));
        }
    }

    Some(lines)
}

#[cfg(target_os = "openbsd")]
fn get_openbsd_gpu_info() -> Option<Vec<String>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pcidump -v | grep -i 'vga'")
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);
    let line = text.lines().next()?;

    let name = if let Some(pos) = line.rfind(':') {
        line[pos + 1..].trim().to_string()
    } else {
        line.trim().to_string()
    };

    if name.is_empty() {
        return None;
    }

    Some(vec![format!("{}: {}", "GPU".bold(), name)])
}

#[cfg(target_os = "netbsd")]
fn get_netbsd_gpu_info() -> Option<Vec<String>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pcictl pci0 list | grep -i 'display'")
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);
    let line = text.lines().next()?;

    let pos = line.find(':')?;
    let name = line[pos + 1..].trim().to_string();

    if name.is_empty() {
        return None;
    }

    let mut lines = vec![format!("{}: {}", "GPU".bold(), name)];

    if let Ok(env_out) = Command::new("envstat").args(["-s", "amdgpu:temperature"]).output() {
        let env_text = String::from_utf8_lossy(&env_out.stdout);
        if let Some(temp_line) = env_text.lines().find(|l| l.contains("degC")) {
            let parts: Vec<&str> = temp_line.split_whitespace().collect();
            if parts.len() >= 2 {
                lines.push(format!("{}: {}°C", "GPU Temp".bold(), parts[1]));
            }
        }
    }

    Some(lines)
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