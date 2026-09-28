use crate::sysinfo::combine::DisplayOptions;
use colored::ColoredString;
use std::fmt::Write;
use std::process::Command;

pub fn get_gpu_info(opts: &DisplayOptions, buf: &mut String, fast_mode: bool, c: fn(&str) -> ColoredString) {
    if !opts.gpu {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", opts.format_header("--- GPU INFO ---", c));
    }

    #[cfg(target_os = "linux")]
    if get_linux_sysfs_gpu(opts, buf, fast_mode, c) {
        return;
    }

    #[cfg(target_os = "linux")]
    if get_nvidia_fast_info(opts, buf, fast_mode, c) {
        return;
    }

    #[cfg(windows)]
    if get_windows_gpu_info(opts, buf, fast_mode, c) {
        return;
    }

    #[cfg(any(target_os = "linux", windows))]
    if !fast_mode && get_nvidia_info(opts, buf, fast_mode, c) {
        return;
    }

    #[cfg(target_os = "macos")]
    if get_macos_gpu_info(opts, buf, c) {
        return;
    }

    #[cfg(target_os = "freebsd")]
    if get_freebsd_gpu_info(opts, buf, c) {
        return;
    }

    #[cfg(target_os = "openbsd")]
    if get_openbsd_gpu_info(opts, buf, c) {
        return;
    }

    #[cfg(target_os = "netbsd")]
    if get_netbsd_gpu_info(opts, buf, c) {
        return;
    }

    if opts.gpu_name {
        let name = clean_gpu_name(&get_generic_gpu_name());
        let _ = writeln!(buf, "{}: {}", c("GPU"), name);
    }
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

    if name.starts_with('[') && name.ends_with(']') {
        name = name[1..name.len() - 1].trim().to_string();
    }

    if name.is_empty() {
        "Unknown GPU".to_string()
    } else {
        name
    }
}

#[cfg(target_os = "linux")]
pub fn get_nvidia_fast_info(
    opts: &DisplayOptions,
    buf: &mut String,
    fast_mode: bool,
    c: fn(&str) -> ColoredString,
) -> bool {
    use std::fs;
    use std::path::Path;

    let gpus_dir = Path::new("/proc/driver/nvidia/gpus");
    if !gpus_dir.exists() { return false; }

    let Ok(entries) = fs::read_dir(gpus_dir) else { return false; };
    let mut found_gpu = false;

    for entry in entries.flatten() {
        let info_path = entry.path().join("information");
        if let Ok(content) = fs::read_to_string(info_path) {
            for line in content.lines() {
                if line.starts_with("Model:")
                    && let Some(pos) = line.find(':')
                {
                    let raw_name = line[pos + 1..].trim();
                    if opts.gpu_name {
                        let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(raw_name));
                    }
                    found_gpu = true;
                    break;
                }
            }
        }
        if found_gpu { break; }
    }

    if !found_gpu { return false; }

    if fast_mode || opts.compact_mode || (!opts.gpu_vram && !opts.gpu_temp) {
        return true;
    }

    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=memory.used,memory.total,temperature.gpu", "--format=csv,noheader,nounits"])
        .output();

    if let Ok(out) = output
        && out.status.success()
    {
        let text = String::from_utf8_lossy(&out.stdout);
        if let Some(line) = text.lines().next() {
            let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if parts.len() == 3 {
                let used_mb: f64 = parts[0].parse().unwrap_or(0.0);
                let total_mb: f64 = parts[1].parse().unwrap_or(0.0);
                let temp = parts[2];
                if opts.gpu_vram {
                    let _ = writeln!(buf, "{}: {:.2} GB / {:.2} GB", c("VRAM"), used_mb / 1024.0, total_mb / 1024.0);
                }
                if opts.gpu_temp {
                    let _ = writeln!(buf, "{}: {}°C", c("GPU Temp"), temp);
                }
            }
        }
    }

    true
}

#[cfg(any(target_os = "linux", windows))]
fn get_nvidia_info(
    opts: &DisplayOptions,
    buf: &mut String,
    fast_mode: bool,
    c: fn(&str) -> ColoredString,
) -> bool {
    let query_args = if fast_mode || opts.compact_mode || (!opts.gpu_vram && !opts.gpu_temp) {
        vec!["--query-gpu=gpu_name", "--format=csv,noheader,nounits"]
    } else {
        vec![
            "--query-gpu=gpu_name,memory.total,memory.used,temperature.gpu",
            "--format=csv,noheader,nounits",
        ]
    };

    #[cfg(target_os = "linux")]
    if !std::path::Path::new("/proc/driver/nvidia").exists() {
        return false;
    }

    let output = Command::new("nvidia-smi")
        .args(&query_args)
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
    if parts.is_empty() {
        return false;
    }

    let name = parts[0];
    if opts.gpu_name {
        let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(name));
    }

    if fast_mode || opts.compact_mode || (!opts.gpu_vram && !opts.gpu_temp) {
        return true;
    }

    if parts.len() >= 4 {
        let mem_total: f64 = parts[1].parse().unwrap_or(0.0) / 1024.0;
        let mem_used: f64 = parts[2].parse().unwrap_or(0.0) / 1024.0;
        let temp = parts[3];

        if opts.gpu_vram {
            let _ = writeln!(
                buf,
                "{}: {:.2} GB / {:.2} GB",
                c("VRAM"),
                mem_used,
                mem_total
            );
        }
        if opts.gpu_temp {
            let _ = writeln!(buf, "{}: {}°C", c("GPU Temp"), temp);
        }
    }

    true
}

#[cfg(target_os = "macos")]
fn get_macos_gpu_info(opts: &DisplayOptions, buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let output = match Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
    {
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

    if opts.gpu_name {
        let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(&gpu_name));
    }
    if !opts.compact_mode && opts.gpu_vram && !vram.is_empty() {
        let _ = writeln!(buf, "{}: {}", c("VRAM"), vram);
    }

    true
}

#[cfg(target_os = "linux")]
fn get_linux_sysfs_gpu(
    opts: &DisplayOptions,
    buf: &mut String,
    fast_mode: bool,
    c: fn(&str) -> ColoredString,
) -> bool {
    use pci_ids::FromId;
    use std::fs;
    use std::path::Path;

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

        let vendor_hex = fs::read_to_string(device_path.join("vendor")).unwrap_or_default();
        let device_hex = fs::read_to_string(device_path.join("device")).unwrap_or_default();

        let vendor_id =
            u16::from_str_radix(vendor_hex.trim().trim_start_matches("0x"), 16).unwrap_or(0);
        let device_id =
            u16::from_str_radix(device_hex.trim().trim_start_matches("0x"), 16).unwrap_or(0);

        if let Some(vendor) = pci_ids::Vendor::from_id(vendor_id) {
            if let Some(device) = vendor.devices().find(|d| d.id() == device_id) {
                gpu_name = device.name().to_string();
            } else {
                gpu_name = format!("{} Graphics", vendor.name());
            }
        }

        if opts.gpu_name {
            let _ = writeln!(buf, "{}: {}", c("GPU"), gpu_name);
        }

        if fast_mode || opts.compact_mode {
            return true;
        }

        if opts.gpu_vram {
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
        }

        if opts.gpu_temp {
            let hwmon_dir = device_path.join("hwmon");
            if let Ok(hwmon_entries) = fs::read_dir(hwmon_dir) {
                for hwmon in hwmon_entries.flatten() {
                    let temp_path = hwmon.path().join("temp1_input");
                    if temp_path.exists()
                        && let Ok(temp_raw) = fs::read_to_string(temp_path)
                        && let Ok(temp_mc) = temp_raw.trim().parse::<f64>()
                    {
                        let _ = writeln!(buf, "{}: {:.0}°C", c("GPU Temp"), temp_mc / 1000.0);
                        break;
                    }
                }
            }
        }

        return true;
    }

    false
}

#[cfg(windows)]
fn get_windows_gpu_info(
    opts: &DisplayOptions,
    buf: &mut String,
    fast_mode: bool,
    c: fn(&str) -> ColoredString,
) -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let video_key_path = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfba-08002be10318}";

    let Ok(video_key) = hklm.open_subkey(video_key_path) else {
        return false;
    };

    let mut found = false;

    for subkey_name in video_key.enum_keys().flatten() {
        if subkey_name.len() != 4 || !subkey_name.chars().all(|ch| ch.is_ascii_digit()) {
            continue;
        }

        if let Ok(gpu_sub_key) = video_key.open_subkey(&subkey_name) {
            if let Ok(driver_desc) = gpu_sub_key.get_value::<String, _>("DriverDesc") {
                let clean_name = clean_gpu_name(&driver_desc);
                if opts.gpu_name {
                    let _ = writeln!(buf, "{}: {}", c("GPU"), clean_name);
                }
                found = true;

                if fast_mode || opts.compact_mode || !opts.gpu_vram {
                    return true;
                }

                if opts.gpu_vram {
                    if let Ok(mem_bytes) = gpu_sub_key.get_value::<u64, _>("HardwareInformation.MemorySize") {
                        let gb = mem_bytes as f64 / 1024.0 / 1024.0 / 1024.0;
                        if gb > 0.0 {
                            let _ = writeln!(buf, "{}: {:.2} GB", c("VRAM"), gb);
                        }
                    } else if let Ok(mem_bytes) = gpu_sub_key.get_value::<u32, _>("HardwareInformation.MemorySize") {
                        let gb = mem_bytes as f64 / 1024.0 / 1024.0 / 1024.0;
                        if gb > 0.0 {
                            let _ = writeln!(buf, "{}: {:.2} GB", c("VRAM"), gb);
                        }
                    }
                }

                break;
            }
        }
    }

    found
}

#[cfg(target_os = "freebsd")]
fn get_freebsd_gpu_info(opts: &DisplayOptions, buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let output = match Command::new("pciconf").arg("-lv").output() {
        Ok(out) => out,
        Err(_) => return false,
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let mut name = String::new();

    for block in text.split("\n\n") {
        if block.to_lowercase().contains("class=0x03") {
            for line in block.lines() {
                if line.trim().starts_with("device") {
                    if let Some(pos) = line.find("='") {
                        name = line[pos + 2..].trim_matches('\'').to_string();
                        break;
                    }
                }
            }
            if !name.is_empty() {
                break;
            }
        }
    }

    if name.is_empty() {
        return false;
    }

    if opts.gpu_name {
        let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(&name));
    }

    if opts.compact_mode || opts.fast_mode || !opts.gpu_temp {
        return true;
    }

    true
}

#[cfg(target_os = "openbsd")]
fn get_openbsd_gpu_info(opts: &DisplayOptions, buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let output = match Command::new("pcidump").arg("-v").output() {
        Ok(out) => out,
        Err(_) => return false,
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let line = match text.lines().find(|l| l.to_lowercase().contains("vga")) {
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

    if opts.gpu_name {
        let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(&name));
    }
    true
}

#[cfg(target_os = "netbsd")]
fn get_netbsd_gpu_info(opts: &DisplayOptions, buf: &mut String, c: fn(&str) -> ColoredString) -> bool {
    let output = match Command::new("pcictl").args(["pci0", "list"]).output() {
        Ok(out) => out,
        Err(_) => return false,
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let line = match text.lines().find(|l| l.to_lowercase().contains("display")) {
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

    if opts.gpu_name {
        let _ = writeln!(buf, "{}: {}", c("GPU"), clean_gpu_name(&name));
    }

    if opts.compact_mode || opts.fast_mode || !opts.gpu_temp {
        return true;
    }

    if let Ok(env_out) = Command::new("envstat")
        .args(["-s", "amdgpu:temperature"])
        .output()
    {
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
        if let Ok(output) = Command::new("system_profiler")
            .arg("SPDisplaysDataType")
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Chipset Model:")
                    && let Some(pos) = trimmed.find(':')
                {
                    return trimmed[pos + 1..].trim().to_string();
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        use pci_ids::FromId;
        use std::fs;
        use std::path::Path;

        let drm_path = Path::new("/sys/class/drm");
        if drm_path.exists()
            && let Ok(entries) = fs::read_dir(drm_path)
        {
            for entry in entries.flatten() {
                let name_str = entry.file_name().to_string_lossy().into_owned();
                if name_str.starts_with("card") && !name_str.contains('-') {
                    let device_path = entry.path().join("device");
                    let vendor_hex =
                        fs::read_to_string(device_path.join("vendor")).unwrap_or_default();
                    let device_hex =
                        fs::read_to_string(device_path.join("device")).unwrap_or_default();

                    let vendor_id =
                        u16::from_str_radix(vendor_hex.trim().trim_start_matches("0x"), 16)
                            .unwrap_or(0);
                    let device_id =
                        u16::from_str_radix(device_hex.trim().trim_start_matches("0x"), 16)
                            .unwrap_or(0);

                    if let Some(vendor) = pci_ids::Vendor::from_id(vendor_id)
                        && let Some(device) = vendor.devices().find(|d| d.id() == device_id)
                    {
                        return device.name().to_string();
                    }
                }
            }
        }
    }

    #[cfg(windows)]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfba-08002be10318}";

        if let Ok(video_key) = hklm.open_subkey(path) {
            for subkey_name in video_key.enum_keys().flatten() {
                if subkey_name.len() == 4 && subkey_name.chars().all(|ch| ch.is_ascii_digit()) {
                    if let Ok(gpu_sub_key) = video_key.open_subkey(&subkey_name) {
                        if let Ok(driver_desc) = gpu_sub_key.get_value::<String, _>("DriverDesc") {
                            if !driver_desc.trim().is_empty() {
                                return driver_desc;
                            }
                        }
                    }
                }
            }
        }
    }

    "Unknown GPU".to_string()
}

pub fn get_primary_gpu_name() -> Option<String> {
    let raw = get_generic_gpu_name();
    let clean = clean_gpu_name(&raw);
    if clean == "Unknown GPU" {
        None
    } else {
        Some(clean)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_gpu_name() {
        assert_eq!(
            clean_gpu_name("NVIDIA Corporation GA106 [GeForce RTX 3060] (rev a1)"),
            "GA106 [GeForce RTX 3060]"
        );
        assert_eq!(
            clean_gpu_name("[GeForce RTX 3060]"),
            "GeForce RTX 3060"
        );
        assert_eq!(
            clean_gpu_name("Advanced Micro Devices, Inc. [AMD/ATI] Navi 22 [Radeon RX 6700/6700 XT/6750 XT]"),
            "Navi 22 [Radeon RX 6700/6700 XT/6750 XT]"
        );
        assert_eq!(clean_gpu_name(""), "Unknown GPU");
    }
}