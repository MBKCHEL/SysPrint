mod config;
mod logos;
mod parser;
mod print;
mod sysinfo;

use clap::Parser;
use parser::Arguments;
use sysinfo::{DisplayOptions, SystemInfo};

fn main() {
    let args = Arguments::parse();

    if args.generate_config {
        match config::generate() {
            Ok(path) => println!("Default config written to {}", path.display()),
            Err(e) => eprintln!("{e}"),
        }
        return;
    }

    // Config is optional; a broken file only warns and falls back to CLI args.
    let cfg = match config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("warning: {e}, ignoring config");
            None
        }
    };

    let config_stronger = cfg.map(|c| c.config_stronger).unwrap_or(false);
    let opts = DisplayOptions {
        system: decide(args.hide_system, cfg.map(|c| c.show_system_info), config_stronger),
        cpu: decide(args.hide_cpu, cfg.map(|c| c.show_cpu_info), config_stronger),
        memory: decide(args.hide_memory, cfg.map(|c| c.show_memory_info), config_stronger),
        disks: decide(args.hide_disks, cfg.map(|c| c.show_disks_info), config_stronger),
        other: decide(args.hide_other, cfg.map(|c| c.show_other_info), config_stronger),
        gpu: decide(args.hide_gpu, cfg.map(|c| c.show_gpu_info), config_stronger),
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

fn decide(flag_hides: bool, config: Option<bool>, config_stronger: bool) -> bool {
    if config_stronger {
        config.unwrap_or(true)
    } else if flag_hides {
        false
    } else {
        config.unwrap_or(true)
    }
}