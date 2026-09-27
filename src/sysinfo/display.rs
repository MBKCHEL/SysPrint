use colored::ColoredString;
use std::fmt::Write;

pub fn get_resolution() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        if let Some(res) = get_linux_resolution() {
            return Some(res);
        }
    }

    #[cfg(windows)]
    {
        if let Some(res) = get_windows_resolution() {
            return Some(res);
        }
    }

    None
}

#[cfg(target_os = "linux")]
fn get_linux_resolution() -> Option<String> {
    use std::fs;
    use std::path::Path;

    let drm_path = Path::new("/sys/class/drm");
    if let Ok(entries) = fs::read_dir(drm_path) {
        let mut resolutions = Vec::new();

        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            // Matches connectors like card0-HDMI-A-1, card1-LVDS-1
            if !file_name.starts_with("card") || !file_name.contains('-') {
                continue;
            }

            let status_path = path.join("status");
            if let Ok(status) = fs::read_to_string(&status_path)
                && status.trim() == "connected"
            {
                let modes_path = path.join("modes");
                if let Ok(modes) = fs::read_to_string(&modes_path)
                    && let Some(first_mode) = modes.lines().next()
                {
                    let mode = first_mode.trim();
                    if !mode.is_empty() && !resolutions.contains(&mode.to_string()) {
                        resolutions.push(mode.to_string());
                    }
                }
            }
        }

        if !resolutions.is_empty() {
            return Some(resolutions.join(", "));
        }
    }

    None
}

#[cfg(windows)]
fn get_windows_resolution() -> Option<String> {
    #[link(name = "user32")]
    extern "system" {
        fn GetSystemMetrics(n_index: i32) -> i32;
    }

    unsafe {
        let w = GetSystemMetrics(0); // SM_CXSCREEN
        let h = GetSystemMetrics(1); // SM_CYSCREEN
        if w > 0 && h > 0 {
            Some(format!("{w}x{h}"))
        } else {
            None
        }
    }
}

pub fn resolution_info(buf: &mut String, c: fn(&str) -> ColoredString) {
    if let Some(res) = get_resolution() {
        let _ = writeln!(buf, "{}: {}", c("Resolution"), res);
    }
}
