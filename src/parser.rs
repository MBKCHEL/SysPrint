//! Argument parsing.
use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    /// Hide GPU info (shown by default)
    #[arg(
        long = "hide-gpu",
        action = clap::ArgAction::SetFalse,
        default_value_t = true
    )]
    pub show_gpu_info: bool,
}
