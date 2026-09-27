//! Argument parsing.
use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    /// Write a default configuration file to the user config directory and exit
    #[arg(long)]
    pub generate_config: bool,

    /// Hide System section
    #[arg(long = "hide-system")]
    pub hide_system: bool,

    /// Hide CPU section
    #[arg(long = "hide-cpu")]
    pub hide_cpu: bool,

    /// Hide Memory section
    #[arg(long = "hide-memory")]
    pub hide_memory: bool,

    /// Hide Disks section
    #[arg(long = "hide-disks")]
    pub hide_disks: bool,

    /// Hide Other section
    #[arg(long = "hide-other")]
    pub hide_other: bool,

    /// Hide GPU section
    #[arg(long = "hide-gpu")]
    pub hide_gpu: bool,

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

    /// Override OS logo by name (e.g. arch, debian, ubuntu, fedora, windows, tux, apple, gentoo)
    #[arg(short = 'l', long = "logo")]
    pub logo: Option<String>,

    /// Do not pause at exit on Windows
    #[arg(long = "no-pause")]
    pub no_pause: bool,
}