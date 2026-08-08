//! Argument parsing.
use clap::{ArgAction, Parser};

#[derive(Parser)]
pub struct Arguments {
    /// Hide System section (shown by default)
    #[arg(
        long = "hide-system",
        action = ArgAction::SetFalse,
        default_value_t = true
    )]
    pub show_system_info: bool,

    /// Hide CPU section (shown by default)
    #[arg(
        long = "hide-cpu",
        action = ArgAction::SetFalse,
        default_value_t = true
    )]
    pub show_cpu_info: bool,

    /// Hide Memory section (shown by default)
    #[arg(
        long = "hide-memory",
        action = ArgAction::SetFalse,
        default_value_t = true
    )]
    pub show_memory_info: bool,

    /// Hide Disks section (shown by default)
    #[arg(
        long = "hide-disks",
        action = ArgAction::SetFalse,
        default_value_t = true
    )]
    pub show_disks_info: bool,

    /// Hide Other section (shown by default)
    #[arg(
        long = "hide-other",
        action = ArgAction::SetFalse,
        default_value_t = true
    )]
    pub show_other_info: bool,

    /// Hide GPU section (shown by default)
    #[arg(
        long = "hide-gpu",
        action = ArgAction::SetFalse,
        default_value_t = true
    )]
    pub show_gpu_info: bool,
}