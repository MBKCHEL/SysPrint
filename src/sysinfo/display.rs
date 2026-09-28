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
pub fn parse_edid_refresh_rate(edid: &[u8]) -> Option<u32> {
    if edid.len() < 72 {
        return None;
    }
    let dtd = &edid[54..72];
    let pixel_clock = ((dtd[0] as u32) | ((dtd[1] as u32) << 8)) * 10_000;
    if pixel_clock == 0 {
        return None;
    }
    let h_active = (dtd[2] as u32) | (((dtd[4] as u32) & 0xf0) << 4);
    let h_blank = (dtd[3] as u32) | (((dtd[4] as u32) & 0x0f) << 8);
    let v_active = (dtd[5] as u32) | (((dtd[7] as u32) & 0xf0) << 4);
    let v_blank = (dtd[6] as u32) | (((dtd[7] as u32) & 0x0f) << 8);

    let h_total = h_active + h_blank;
    let v_total = v_active + v_blank;
    if h_total == 0 || v_total == 0 {
        return None;
    }

    let rate = (pixel_clock as f32) / ((h_total * v_total) as f32);
    if (23.0..=360.0).contains(&rate) {
        Some(rate.round() as u32)
    } else {
        None
    }
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
                    if !mode.is_empty() {
                        let res_string = if let Ok(edid_bytes) = fs::read(path.join("edid"))
                            && let Some(hz) = parse_edid_refresh_rate(&edid_bytes)
                        {
                            format!("{mode} @ {hz}Hz")
                        } else {
                            mode.to_string()
                        };

                        if !resolutions.contains(&res_string) {
                            resolutions.push(res_string);
                        }
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
        fn GetDC(hwnd: usize) -> usize;
        fn ReleaseDC(hwnd: usize, hdc: usize) -> i32;
    }

    #[link(name = "gdi32")]
    extern "system" {
        fn GetDeviceCaps(hdc: usize, index: i32) -> i32;
    }

    unsafe {
        let w = GetSystemMetrics(0); // SM_CXSCREEN
        let h = GetSystemMetrics(1); // SM_CYSCREEN
        if w > 0 && h > 0 {
            let hdc = GetDC(0);
            let hz = if hdc != 0 {
                let vrefresh = GetDeviceCaps(hdc, 116); // VREFRESH
                ReleaseDC(0, hdc);
                vrefresh
            } else {
                0
            };

            if hz > 1 {
                Some(format!("{w}x{h} @ {hz}Hz"))
            } else {
                Some(format!("{w}x{h}"))
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_edid_refresh_rate() {
        let mut edid = vec![0u8; 128];
        // DTD at offset 54:
        // pixel clock: 7640 -> 0x1DD8
        edid[54] = 0xd8;
        edid[55] = 0x1d;
        // h_active: 1366 (0x556) -> low: 0x56, high: 0x5
        edid[56] = 0x56;
        // h_blank: 194 (0x0C2) -> low: 0xC2, high: 0x0
        edid[57] = 0xc2;
        // byte 58: high 4 bits h_active (0x5), low 4 bits h_blank (0x0) -> 0x50
        edid[58] = 0x50;
        // v_active: 768 (0x300) -> low: 0x00, high: 0x3
        edid[59] = 0x00;
        // v_blank: 48 (0x030) -> low: 0x30, high: 0x0
        edid[60] = 0x30;
        // byte 61: high 4 bits v_active (0x3), low 4 bits v_blank (0x0) -> 0x30
        edid[61] = 0x30;

        let hz = parse_edid_refresh_rate(&edid);
        assert_eq!(hz, Some(60));
    }
}
