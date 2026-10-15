//! Configuration file handling (`config.toml`).
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
const CONFIG_DIR_NAME: &str = "sysprint";
const FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Config {
    pub logo: String,
    pub logo_color: String,
    pub config_stronger: bool,
    pub mini_logo_mode: bool,
    pub fast_mode: bool,
    pub compact_mode: bool,
    pub hide_fetch_info: bool,
    pub show_sysprint_start_time: bool,

    pub system: SystemConfig,
    pub cpu: CpuConfig,
    pub gpu: GpuConfig,
    pub memory: MemoryConfig,
    pub other: OtherConfig,
    pub disks: DisksConfig,
}

// --- SYSTEM ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct SystemConfig {
    pub enabled: bool,
    pub os_name: bool,
    pub kernel: bool,
    pub os_version: bool,
    pub init: bool,
    pub host: bool,
    pub user: bool,
    pub uptime: bool,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            os_name: true,
            kernel: true,
            os_version: true,
            init: true,
            host: true,
            user: true,
            uptime: true,
        }
    }
}

// --- CPU ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct CpuConfig {
    pub enabled: bool,
    pub name: bool,
    pub ghz: bool,
    pub usage: bool,
    pub temp: bool,
    pub cores: bool,
    pub threads: bool,
    pub architecture: bool,
}

impl Default for CpuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            name: true,
            ghz: true,
            usage: true,
            temp: true,
            cores: true,
            threads: true,
            architecture: true,
        }
    }
}

// --- GPU ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct GpuConfig {
    pub enabled: bool,
    pub name: bool,
    pub vram: bool,
    pub temp: bool,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            name: true,
            vram: true,
            temp: true,
        }
    }
}

// --- MEMORY ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct MemoryConfig {
    pub enabled: bool,
    pub ram: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ram: true,
        }
    }
}

// --- OTHER ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct OtherConfig {
    pub enabled: bool,
    pub de: bool,
    pub wm: bool,
    pub terminal: bool,
    pub shell: bool,
    pub battery: bool,
    pub locale_time: bool,
}

impl Default for OtherConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            de: true,
            wm: true,
            terminal: true,
            shell: true,
            battery: true,
            locale_time: true,
        }
    }
}

// --- DISKS ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct DisksConfig {
    pub enabled: bool,
    pub list: bool,
}

impl Default for DisksConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            list: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logo: String::from("default"),
            logo_color: String::from("default"),
            config_stronger: false,
            mini_logo_mode: false,
            fast_mode: false,
            compact_mode: false,
            hide_fetch_info: false,
            show_sysprint_start_time: true,
            system: SystemConfig::default(),
            cpu: CpuConfig::default(),
            gpu: GpuConfig::default(),
            memory: MemoryConfig::default(),
            other: OtherConfig::default(),
            disks: DisksConfig::default(),
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
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
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

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create directory {}: {e}", parent.display()))?;
    }

    let contents = format!(
        "# SysPrint configuration\n\
         # When a CLI flag contradicts the config, `config-stronger = true` makes the config win.\n\
         # Fast-mode disable GPU Vram and Temp info.\n\n\
         {}\n",
        toml::to_string_pretty(&Config::default()).map_err(|e| e.to_string())?
    );

    fs::write(&path, contents).map_err(|e| format!("failed to write {}: {e}", path.display()))?;
    Ok(path)
}