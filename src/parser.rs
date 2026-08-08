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
}