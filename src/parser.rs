//! Argument parsing.
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    /// Write a default configuration file to the user config directory and exit
    #[arg(long)]
    pub generate_config: bool,

    /// Print logos
    #[arg(long)]
    pub logo: Option<String>,

    /// Logo Color
    #[arg(long)]
    pub logo_color: Option<String>,

    /// Path to custom logo
    #[arg(long)]
    pub custom_logo_path: Option<String>,

    /// Hide System section
    #[arg(long = "hide-system")]
    pub hide_system: bool,

    /// Hide CPU section
    #[arg(long = "hide-cpu")]
    pub hide_cpu: bool,

    /// Hide GPU section
    #[arg(long = "hide-gpu")]
    pub hide_gpu: bool,

    /// Hide Memory section
    #[arg(long = "hide-memory")]
    pub hide_memory: bool,

    /// Hide Other section
    #[arg(long = "hide-other")]
    pub hide_other: bool,

    /// Hide Disks section
    #[arg(long = "hide-disks")]
    pub hide_disks: bool,

    /// Mini mode
    #[arg(short = 'm', long = "mini")]
    pub mini: bool,

    /// Fast mode
    #[arg(short = 'f', long = "fast-mode")]
    pub fast_mode: bool,

    /// Compact mode
    #[arg(short = 'c', long = "compact-mode")]
    pub compact_mode: bool,

    /// Hide "Fetch: SysPrint"
    #[arg(long = "hide-fetch-info")]
    pub hide_fetch_info: bool,

    #[arg(long = "show-sysprint-start-time")]
    pub show_sysprint_start_time: bool,
}
