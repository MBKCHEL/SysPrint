//! Argument parsing.
use argh::FromArgs;
/// Struct with 'Arguments'
#[derive(FromArgs)]
pub struct Arguments {
    /// write a default configuration file to the user config directory and exit
    #[argh(switch)]
    pub generate_config: bool,

    /// print logos
    #[argh(option)]
    pub logo: Option<String>,

    /// logo color
    #[argh(option)]
    pub logo_color: Option<String>,

    /// path to custom logo
    #[argh(option)]
    pub custom_logo_path: Option<String>,

    /// hide system section
    #[argh(switch)]
    pub hide_system: bool,

    /// hide cpu section
    #[argh(switch)]
    pub hide_cpu: bool,

    /// hide gpu section
    #[argh(switch)]
    pub hide_gpu: bool,

    /// hide memory section
    #[argh(switch)]
    pub hide_memory: bool,

    /// hide other section
    #[argh(switch)]
    pub hide_other: bool,

    /// hide disks section
    #[argh(switch)]
    pub hide_disks: bool,

    /// fast mode
    #[argh(switch)]
    pub fast_mode: bool,

    /// compact mode
    #[argh(switch)]
    pub compact_mode: bool,

    /// show speed 'SysPrint'
    #[argh(switch)]
    pub show_sysprint_start_time: bool,
}
