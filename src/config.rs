//! Configuration file handling (`.sysinfo.toml`).
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const FILE_NAME: &str = ".sysinfo.toml";

/// Which sections are enabled.
///
/// `config_stronger` (TOML key `config-stronger`) decides the winner of a
/// conflict between a CLI flag and the config file:
/// - `true` — the config value always wins;
/// - `false` — an explicitly passed CLI flag wins (config is the fallback).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub show_system_info: bool,
    pub show_cpu_info: bool,
    pub show_memory_info: bool,
    pub show_disks_info: bool,
    pub show_other_info: bool,
    pub show_gpu_info: bool,
    #[serde(default)]
    pub config_stronger: bool,
    #[serde(default)]
    pub mini_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_system_info: true,
            show_cpu_info: true,
            show_memory_info: true,
            show_disks_info: true,
            show_other_info: true,
            show_gpu_info: true,
            config_stronger: false,
            mini_mode: false,
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .map(|dir| dir.join(FILE_NAME))
        .unwrap_or_else(|| PathBuf::from(FILE_NAME))
}

pub fn load() -> Result<Option<Config>, String> {
    let path = config_path();
    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // if don`t have config
            let _ = generate();
            return Ok(Some(Config::default()));
        }
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

    let contents = format!(
        "# SysPrint configuration\n\
         # When a CLI flag contradicts the config, `config-stronger = true` makes the config win.\n\
         {}\n",
        toml::to_string_pretty(&Config::default()).map_err(|e| e.to_string())?
    );

    fs::write(&path, contents).map_err(|e| format!("failed to write {}: {e}", path.display()))?;
    Ok(path)
}