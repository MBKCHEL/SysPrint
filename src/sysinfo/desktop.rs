use colored::ColoredString;
use std::env;
use std::fmt::Write;

pub fn get_de() -> String {
    if cfg!(windows) {
        "Explorer".to_string()
    } else {
        env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string())
    }
}

pub fn de_check(buf: &mut String, c: fn(&str) -> ColoredString) {
    let desktop = get_de();
    let _ = writeln!(buf, "{}: {}", c("DE"), desktop);
}

pub fn get_wm() -> String {
    let wm = if cfg!(target_os = "windows") {
        if env::var("GLAZEWM_VERSION").is_ok() {
            "GlazeWM".to_string()
        } else if env::var("KOMOREBI_CONFIG_HOME").is_ok() {
            "Komorebi".to_string()
        } else {
            "Explorer.exe".to_string()
        }
    } else if cfg!(target_os = "macos") {
        if env::var("YABAI_SOCKET").is_ok() {
            "Yabai".to_string()
        } else if env::var("AMETHYST_VERSION").is_ok() {
            "Amethyst".to_string()
        } else {
            "Quartz Compositor".to_string()
        }
    } else if env::var("NIRI_SOCKET").is_ok() {
        "Niri".to_string()
    } else if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        "Hyprland".to_string()
    } else if env::var("I3SOCK").is_ok() {
        "i3".to_string()
    } else if env::var("SWAYSOCK").is_ok() {
        "Sway".to_string()
    } else if env::var("BSPWM_SOCKET").is_ok() {
        "bspwm".to_string()
    } else if env::var("HERBSTLUFTWM_SOCKET").is_ok() {
        "herbstluftwm".to_string()
    } else if let Ok(wm_env) = env::var("WINDOWMANAGER") {
        wm_env.split('/').next_back().unwrap_or(&wm_env).to_string()
    } else {
        let desktop = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_default()
            .to_lowercase();

        if desktop.contains("niri") {
            "niri".to_string()
        } else if desktop.contains("kde") || env::var("KDE_FULL_SESSION").is_ok() {
            "KWin".to_string()
        } else if desktop.contains("gnome") {
            "Mutter".to_string()
        } else if desktop.contains("xfce") {
            "Xfwm4".to_string()
        } else if desktop.contains("cinnamon") {
            "Muffin".to_string()
        } else if desktop.contains("mate") {
            "Marco".to_string()
        } else if !desktop.is_empty() {
            desktop
        } else {
            "Unknown".to_string()
        }
    };

    #[cfg(not(windows))]
    if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
        let session_lower = session_type.to_lowercase();
        let protocol = match session_lower.as_str() {
            "wayland" => "Wayland",
            "x11" => "X11",
            "tty" => "TTY",
            other => other,
        };
        format!("{} ({})", wm, protocol)
    } else {
        wm
    }

    #[cfg(windows)]
    wm
}

pub fn wm_check(buf: &mut String, c: fn(&str) -> ColoredString) {
    let wm_display = get_wm();
    let _ = writeln!(buf, "{}: {}", c("WM"), wm_display);
}
