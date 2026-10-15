mod config;
mod logos;
mod parser;
mod print;
mod sysinfo;

use crate::config::Config;
use crate::sysinfo::combine::DisplayOptions;
use crate::sysinfo::combine::SystemInfo;
use clap::Parser;
use parser::Arguments;

fn main() {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let args = Arguments::parse();

    if args.generate_config {
        match config::generate() {
            Ok(path) => println!("Default config written to {}", path.display()),
            Err(e) => eprintln!("{e}"),
        }
        return;
    }

    let cfg = match config::load() {
        Ok(cfg) => cfg.unwrap_or_default(),
        Err(e) => {
            eprintln!("warning: {e}, ignoring config");
            Config::default()
        }
    };

    let config_stronger = cfg.config_stronger;

    let logo_cfg = decide_string(args.logo, cfg.logo, config_stronger);
    let logo_color_cfg = decide_string(args.logo_color, cfg.logo_color, config_stronger);

    let mut system_cfg = cfg.system;
    system_cfg.enabled = decide(args.hide_system, system_cfg.enabled, config_stronger);

    let mut cpu_cfg = cfg.cpu;
    cpu_cfg.enabled = decide(args.hide_cpu, cpu_cfg.enabled, config_stronger);

    let mut gpu_cfg = cfg.gpu;
    gpu_cfg.enabled = decide(args.hide_gpu, gpu_cfg.enabled, config_stronger);

    let mut memory_cfg = cfg.memory;
    memory_cfg.enabled = decide(args.hide_memory, memory_cfg.enabled, config_stronger);

    let mut other_cfg = cfg.other;
    other_cfg.enabled = decide(args.hide_other, other_cfg.enabled, config_stronger);

    let mut disks_cfg = cfg.disks;
    disks_cfg.enabled = decide(args.hide_disks, disks_cfg.enabled, config_stronger);

    let opts = DisplayOptions {
        system: system_cfg,
        cpu: cpu_cfg,
        gpu: gpu_cfg,
        memory: memory_cfg,
        other: other_cfg,
        disks: disks_cfg,
        logo: logo_cfg,
        logo_color: logo_color_cfg,
        mini_logo_mode: decide_bool(args.mini, cfg.mini_logo_mode, config_stronger),
        fast_mode: decide_bool(args.fast_mode, cfg.fast_mode, config_stronger),
        compact_mode: decide_bool(args.compact_mode, cfg.compact_mode, config_stronger),
        hide_fetch_info: decide_bool(args.hide_fetch_info, cfg.hide_fetch_info, config_stronger),
        show_sysprint_start_time: decide_bool(
            args.show_sysprint_start_time,
            cfg.show_sysprint_start_time,
            config_stronger,
        ),
    };

    let info = SystemInfo::collect(opts);
    print::render(&info);

    #[cfg(windows)]
    {
        use std::io::stdin;
        let mut dummy = String::new();
        let _ = stdin().read_line(&mut dummy);
    }
}

fn decide(hide_flag: bool, config_val: bool, config_stronger: bool) -> bool {
    if config_stronger {
        config_val
    } else if hide_flag {
        false
    } else {
        config_val
    }
}

fn decide_bool(flag: bool, config_val: bool, config_stronger: bool) -> bool {
    if config_stronger {
        config_val
    } else if flag {
        true
    } else {
        config_val
    }
}
fn decide_string(cli_val: Option<String>, config_val: String, config_stronger: bool) -> String {
    if config_stronger {
        config_val
    } else {
        cli_val.unwrap_or(config_val)
    }
}