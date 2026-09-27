//! Configuration file handling (`config.toml`).
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const CONFIG_DIR_NAME: &str = "sysprint";
const FILE_NAME: &str = "config.toml";

/// Which sections are enabled.
///
/// `config_stronger` (TOML key `config-stronger`) decides the winner of a
/// conflict between a CLI flag and the config file:
/// - `true` — the config value always wins;
/// - `false` — an explicitly passed CLI flag wins (config is the fallback).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct Config {
    // Custom logo override by name (e.g. "arch", "debian", "ubuntu", "fedora", "windows", "tux")
    pub logo: String,

    // Color customization
    pub accent_color: String,
    pub header_color: String,

    // Modes & behavior
    pub mini_logo_mode: bool,
    pub fast_mode: bool,
    pub compact_mode: bool,
    pub show_progress_bars: bool,
    pub no_logo: bool,
    pub show_time: bool,
    pub config_stronger: bool,

    // Master section toggles
    pub show_system_info: bool,
    pub show_cpu_info: bool,
    pub show_gpu_info: bool,
    pub show_memory_info: bool,
    pub show_other_info: bool,
    pub show_disks_info: bool,

    // Granular System toggles
    pub show_os: bool,
    pub show_kernel: bool,
    pub show_os_version: bool,
    pub show_init: bool,
    pub show_host: bool,
    pub show_user: bool,
    pub show_uptime: bool,
    pub show_processes: bool,

    // Granular CPU toggles
    pub show_cpu_name: bool,
    pub show_cpu_freq: bool,
    pub show_cpu_usage: bool,
    pub show_cpu_temp: bool,
    pub show_cpu_cores: bool,
    pub show_cpu_arch: bool,

    // Granular GPU toggles
    pub show_gpu_name: bool,
    pub show_gpu_temp: bool,
    pub show_gpu_vram: bool,

    // Granular Memory toggles
    pub show_ram: bool,
    pub show_swap: bool,

    // Granular Other toggles
    pub show_de: bool,
    pub show_wm: bool,
    pub show_terminal: bool,
    pub show_shell: bool,
    pub show_resolution: bool,
    pub show_local_ip: bool,
    pub show_battery: bool,
    pub show_locale_time: bool,
    pub show_fetch_info: bool,

    // Granular Disks toggles
    pub show_disks: bool,
    pub show_all_disks: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logo: String::new(),
            accent_color: "auto".to_string(),
            header_color: "auto".to_string(),
            mini_logo_mode: false,
            fast_mode: false,
            compact_mode: false,
            show_progress_bars: false,
            no_logo: false,
            show_time: false,
            config_stronger: false,

            show_system_info: true,
            show_cpu_info: true,
            show_gpu_info: true,
            show_memory_info: true,
            show_other_info: true,
            show_disks_info: true,

            show_os: true,
            show_kernel: true,
            show_os_version: true,
            show_init: true,
            show_host: true,
            show_user: true,
            show_uptime: true,
            show_processes: true,

            show_cpu_name: true,
            show_cpu_freq: true,
            show_cpu_usage: false,
            show_cpu_temp: true,
            show_cpu_cores: true,
            show_cpu_arch: true,

            show_gpu_name: true,
            show_gpu_temp: true,
            show_gpu_vram: true,

            show_ram: true,
            show_swap: true,

            show_de: true,
            show_wm: true,
            show_terminal: true,
            show_shell: true,
            show_resolution: true,
            show_local_ip: true,
            show_battery: true,
            show_locale_time: true,
            show_fetch_info: true,
            show_disks: true,
            show_all_disks: false,
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .map(|dir| dir.join(CONFIG_DIR_NAME).join(FILE_NAME))
        .unwrap_or_else(|| PathBuf::from(FILE_NAME))
}

pub fn load() -> Result<Option<Config>, String> {
    let path = config_path();
    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("failed to read {}: {e}", path.display())),
    };

    match toml::from_str::<Config>(&contents) {
        Ok(cfg) => Ok(Some(cfg)),
        Err(e) => Err(format!("cannot parse {}: {e}", path.display())),
    }
}

pub fn generate() -> Result<PathBuf, String> {
    let path = config_path();

    if path.exists() {
        return Err(format!(
            "config file {} already exists, refusing to overwrite",
            path.display()
        ));
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create directory {}: {e}", parent.display()))?;
    }

    let contents = r#"# SysPrint Configuration

# Custom logo override by name (e.g. "arch", "debian", "ubuntu", "fedora", "windows", "tux", "apple", "gentoo", etc.)
# Leave empty for automatic OS detection.
logo = ""

# Color customization
# Supported: "auto", "cyan", "blue", "green", "red", "magenta", "yellow", "white", "black"
# "auto" uses the default color of the current distribution's logo.
accent-color = "auto"
header-color = "auto"

# Modes & behavior
mini-logo-mode = false
fast-mode = false
compact-mode = false
show-progress-bars = false
no-logo = false
show-time = false
config-stronger = false

# Master section toggles
show-system-info = true
show-cpu-info = true
show-gpu-info = true
show-memory-info = true
show-other-info = true
show-disks-info = true

# Granular: System
show-os = true
show-kernel = true
show-os-version = true
show-init = true
show-host = true
show-user = true
show-uptime = true
show-processes = true

# Granular: CPU
show-cpu-name = true
show-cpu-freq = true
show-cpu-usage = false # Measuring CPU usage takes ~200ms
show-cpu-temp = true
show-cpu-cores = true
show-cpu-arch = true

# Granular: GPU
show-gpu-name = true
show-gpu-temp = true
show-gpu-vram = true

# Granular: Memory
show-ram = true
show-swap = true

# Granular: Other
show-de = true
show-wm = true
show-terminal = true
show-shell = true
show-resolution = true
show-local-ip = true
show-battery = true
show-locale-time = true
show-fetch-info = true

# Granular: Disks
show-disks = true
show-all-disks = false # Set true to show pseudo, loop, and container filesystems
"#;


    fs::write(&path, contents).map_err(|e| format!("failed to write {}: {e}", path.display()))?;

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let cfg = Config::default();
        assert!(cfg.show_system_info);
        assert!(cfg.show_cpu_info);
        assert!(cfg.show_disks);
        assert_eq!(cfg.logo, "");
    }

    #[test]
    fn test_config_deserialize_partial() {
        let toml_str = r#"
            logo = "debian"
            config-stronger = true
            show-cpu-temp = false
            show-disks = false
        "#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.logo, "debian");
        assert!(cfg.config_stronger);
        assert!(!cfg.show_cpu_temp);
        assert!(!cfg.show_disks);
        // Defaults preserved
        assert!(cfg.show_cpu_name);
        assert!(cfg.show_system_info);
    }

    #[test]
    fn test_deny_unknown_fields() {
        let toml_str = r#"
            show-cpu-tempp = false
        "#;
        assert!(toml::from_str::<Config>(toml_str).is_err());
    }
}