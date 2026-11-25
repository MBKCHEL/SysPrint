//! Configuration file handling (`config.toml`).
use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::path::PathBuf;
const CONFIG_DIR_NAME: &str = "sysprint";
const FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Config {
    pub logo: String,
    pub logo_color: String,
    pub custom_logo_path: String,
    pub config_stronger: bool,
    pub mini_logo_mode: bool,
    pub fast_mode: bool,
    pub compact_mode: bool,
    pub hide_fetch_info: bool,
    pub show_sysprint_start_time: bool,

    pub order: OrderConfig,
    pub system: SystemConfig,
    pub cpu: CpuConfig,
    pub gpu: GpuConfig,
    pub memory: MemoryConfig,
    pub other: OtherConfig,
    pub disks: DisksConfig,
}


// Order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct OrderConfig {
    pub order: Vec<String>,
}

impl Default for OrderConfig {
    fn default() -> Self {
        Self {
            order: vec![
                "system".to_string(),
                "cpu".to_string(),
                "gpu".to_string(),
                "memory".to_string(),
                "other".to_string(),
                "disks".to_string(),
            ],
        }
    }
}

// --- SYSTEM ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct SystemConfig {
    pub enabled: bool,
    pub header_color_system: String,
    pub header_enabled_system: bool,
    pub os_name: bool,
    pub kernel: bool,
    pub arch: bool,
    pub os_version: bool,
    pub init: bool,
    pub host: bool,
    pub user: bool,
    pub uptime: bool,
    pub order_system: Vec<String>,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            header_color_system: String::from("cyan"),
            header_enabled_system: true,
            os_name: true,
            kernel: true,
            arch: true,
            os_version: true,
            init: true,
            host: true,
            user: true,
            uptime: true,
            order_system: vec![
                "os-name".to_string(),
                "kernel".to_string(),
                "os-version".to_string(),
                "init".to_string(),
                "host".to_string(),
                "user".to_string(),
                "uptime".to_string(),
            ]
        }
    }
}

// --- CPU ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct CpuConfig {
    pub enabled: bool,
    pub header_color_cpu: String,
    pub header_enabled_cpu: bool,
    pub name: bool,
    pub ghz: bool,
    pub usage: bool,
    pub temp: bool,
    pub cores: bool,
    pub threads: bool,
    pub order_cpu: Vec<String>,
}

impl Default for CpuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            header_color_cpu: String::from("cyan"),
            header_enabled_cpu: true,
            name: true,
            ghz: true,
            usage: true,
            temp: true,
            cores: true,
            threads: true,
            order_cpu: vec![
                "cpu-name".to_string(),
                "cpu-ghz".to_string(),
                "cpu-usage".to_string(),
                "cpu-temp".to_string(),
                "cpu-cores".to_string(),
                "cpu-threads".to_string(),
            ],
        }
    }
}

// --- GPU ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct GpuConfig {
    pub enabled: bool,
    pub header_color_gpu: String,
    pub header_enabled_gpu: bool,
    pub name: bool,
    pub vram: bool,
    pub temp: bool,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            header_color_gpu: String::from("cyan"),
            header_enabled_gpu: true,
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
    pub header_color_memory: String,
    pub header_enabled_memory: bool,
    pub ram: bool,
    pub ram_used: bool,
    pub ram_total: bool,
    pub swap_used: bool,
    pub swap_total: bool,
    pub swap: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            header_color_memory: String::from("cyan"),
            header_enabled_memory: true,
            ram: true,
            swap: true,
            ram_used: true,
            ram_total: true,
            swap_used: true,
            swap_total: true,
        }
    }
}

// --- OTHER ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct OtherConfig {
    pub enabled: bool,
    pub header_color_other: String,
    pub header_enabled_other: bool,
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
            header_color_other: String::from("cyan"),
            header_enabled_other: true,
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
    pub header_color_disk: String,
    pub header_enabled_disk: bool,
    pub list: bool,
}

impl Default for DisksConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            header_color_disk: String::from("cyan"),
            header_enabled_disk: true,
            list: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logo: String::from("default"),
            logo_color: String::from("default"),
            custom_logo_path: String::from("none"),
            config_stronger: false,
            mini_logo_mode: false,
            fast_mode: false,
            compact_mode: false,
            hide_fetch_info: false,
            show_sysprint_start_time: true,
            order: OrderConfig::default(),
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
    let base_dir = {
        #[cfg(target_os = "windows")]
        {
            // %APPDATA% -> C:\Users\<User>\AppData\Roaming
            env::var_os("APPDATA")
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
        }
        #[cfg(target_os = "macos")]
        {
            // ~/Library/Application Support
            env::var_os("HOME")
                .filter(|s| !s.is_empty())
                .map(|h| PathBuf::from(h).join("Library/Application Support"))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            // Linux, BSD tree
            env::var_os("XDG_CONFIG_HOME")
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
                .or_else(|| {
                    env::var_os("HOME")
                        .filter(|s| !s.is_empty())
                        .map(|h| PathBuf::from(h).join(".config"))
                })
        }
    };

    base_dir
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
        Ok(cfg) => {
            let updated_contents = format!(
                "# SysPrint configuration\n\
                 # When a CLI flag contradicts the config, `config-stronger = true` makes the config win.\n\
                 # Fast-mode disable GPU Vram and Temp info.\n\
                 # [order] it`s vec with order info point. If order full empty (even quotes prints default).\n\
                 {}\n",
                toml::to_string_pretty(&cfg).map_err(|e| e.to_string())?
            );
            let _ = fs::write(&path, updated_contents);

            Ok(Some(cfg))
        }
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
