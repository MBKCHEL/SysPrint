use crate::sysinfo::combine::DisplayOptions;
use colored::ColoredString;
use std::fmt::Write;

pub fn get_battery_raw() -> Option<(u8, Option<String>)> {
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

                    if let Ok(pct) = cap.trim().parse::<u8>() {
                        let status_opt = if stat.trim().is_empty() {
                            None
                        } else {
                            Some(stat.trim().to_string())
                        };
                        return Some((pct, status_opt));
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
                return Some((status.battery_life_percent, None));
            }
        }
    }

    None
}

pub fn battery_info(opts: &DisplayOptions, buf: &mut String, c: fn(&str) -> ColoredString) {
    if let Some((percent, status)) = get_battery_raw() {
        let status_str = match status {
            Some(s) => format!(" [{s}]"),
            None => String::new(),
        };
        let bar = if opts.progress_bars {
            format!("{} ", crate::sysinfo::combine::make_bar(percent as f64, 10, c))
        } else {
            String::new()
        };
        let _ = writeln!(buf, "{}: {}{}%{}", c("Battery"), bar, percent, status_str);
    }
}
