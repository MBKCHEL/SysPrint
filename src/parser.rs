//! Argument parsing.
use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Arguments {
    /// Write a default configuration file to the user config directory and exit
    #[arg(long)]
    pub generate_config: bool,

    /// Print the path to the configuration file and exit
    #[arg(long = "config-path")]
    pub config_path: bool,

    /// Hide System section
    #[arg(long = "hide-system")]
    pub hide_system: bool,

    /// Hide CPU section
    #[arg(long = "hide-cpu")]
    pub hide_cpu: bool,

    /// Display CPU usage percentage (requires ~200ms sample delay)
    #[arg(long = "cpu-usage")]
    pub cpu_usage: bool,

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

    /// Display visual progress bars for RAM, Swap, CPU, Disks, and Battery
    #[arg(short = 'b', long = "bars")]
    pub bars: bool,

    /// Display all mount points including pseudo, loop, and container filesystems
    #[arg(long = "all-disks")]
    pub all_disks: bool,

    /// Hide "Fetch: SysPrint"
    #[arg(long = "hide-fetch-info")]
    pub hide_fetch_info: bool,

    /// Override OS logo by name (e.g. arch, debian, ubuntu, fedora, windows, tux, apple, gentoo)
    #[arg(short = 'l', long = "logo")]
    pub logo: Option<String>,

    /// Override accent color (auto, cyan, blue, green, red, magenta, yellow, white, black)
    #[arg(short = 'C', long = "color")]
    pub color: Option<String>,

    /// Output all gathered system information as JSON and exit
    #[arg(long)]
    pub json: bool,

    /// Display execution time in milliseconds next to SysPrint version
    #[arg(long = "time")]
    pub time: bool,

    /// Do not display ASCII logo
    #[arg(long = "no-logo")]
    pub no_logo: bool,

    /// Do not pause at exit on Windows
    #[arg(long = "no-pause")]
    pub no_pause: bool,
}