use colored::ColoredString;
use std::fmt::Write;

pub fn battery_info(buf: &mut String, c: fn(&str) -> ColoredString) {
    let mut battery_str = String::new();

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("BAT") {
                    let path = entry.path();
                    let cap = fs::read_to_string(path.join("capacity")).unwrap_or_default();
                    let stat = fs::read_to_string(path.join("status")).unwrap_or_default();

                    if !cap.trim().is_empty() {
                        let status_str = if stat.trim().is_empty() {
                            "".to_string()
                        } else {
                            format!(" [{}]", stat.trim())
                        };
                        battery_str = format!("{}%{}", cap.trim(), status_str);
                        break;
                    }
                }
            }
        }
    }

    #[cfg(windows)]
    {
        #[repr(C)]
        #[derive(Default)]
        struct SystemPowerStatus {
            ac_line_status: u8,
            battery_flag: u8,
            battery_life_percent: u8,
            system_status_flag: u8,
            battery_life_time: u32,
            battery_full_life_time: u32,
        }

        unsafe extern "system" {
            fn GetSystemPowerStatus(status: *mut SystemPowerStatus) -> i32;
        }

        let mut status = SystemPowerStatus::default();

        unsafe {
            if GetSystemPowerStatus(&mut status) != 0 && status.battery_life_percent != 255 {
                battery_str = format!("{}%", status.battery_life_percent);
            }
        }
    }

    if !battery_str.is_empty() {
        let _ = writeln!(buf, "{}: {}", c("Battery"), battery_str);
    }
}
