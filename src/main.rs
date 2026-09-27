mod config;
mod logos;
mod parser;
mod print;
mod sysinfo;

use crate::sysinfo::combine::DisplayOptions;
use crate::sysinfo::combine::SystemInfo;
use clap::Parser;
use parser::Arguments;

fn main() {
    // Enable ANSI escape codes for Windows console
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

    // Config is optional; a broken file only warns and falls back to CLI args.
    let cfg = match config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("warning: {e}, ignoring config");
            None
        }
    };

    let config_stronger = cfg.as_ref().map(|c| c.config_stronger).unwrap_or(false);

    let raw_logo = if config_stronger {
        cfg.as_ref()
            .map(|c| c.logo.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .or(args.logo)
    } else {
        args.logo.or_else(|| {
            cfg.as_ref()
                .map(|c| c.logo.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        })
    };

    let custom_logo = if let Some(logo) = raw_logo {
        if logos::is_valid_logo(&logo) {
            Some(logo)
        } else {
            eprintln!("warning: unknown logo '{logo}', falling back to default");
            None
        }
    } else {
        None
    };

    let opts = DisplayOptions {
        custom_logo,
        mini_logo_mode: decide_enable(args.mini, cfg.as_ref().map(|c| c.mini_logo_mode), config_stronger),
        fast_mode: decide_enable(args.fast_mode, cfg.as_ref().map(|c| c.fast_mode), config_stronger),
        compact_mode: decide_enable(args.compact_mode, cfg.as_ref().map(|c| c.compact_mode), config_stronger),

        system: decide_show(args.hide_system, cfg.as_ref().map(|c| c.show_system_info), config_stronger),
        cpu: decide_show(args.hide_cpu, cfg.as_ref().map(|c| c.show_cpu_info), config_stronger),
        gpu: decide_show(args.hide_gpu, cfg.as_ref().map(|c| c.show_gpu_info), config_stronger),
        memory: decide_show(args.hide_memory, cfg.as_ref().map(|c| c.show_memory_info), config_stronger),
        other: decide_show(args.hide_other, cfg.as_ref().map(|c| c.show_other_info), config_stronger),
        disks: decide_show(args.hide_disks, cfg.as_ref().map(|c| c.show_disks_info), config_stronger),

        os: cfg.as_ref().map(|c| c.show_os).unwrap_or(true),
        kernel: cfg.as_ref().map(|c| c.show_kernel).unwrap_or(true),
        os_version: cfg.as_ref().map(|c| c.show_os_version).unwrap_or(true),
        init: cfg.as_ref().map(|c| c.show_init).unwrap_or(true),
        host: cfg.as_ref().map(|c| c.show_host).unwrap_or(true),
        user: cfg.as_ref().map(|c| c.show_user).unwrap_or(true),
        uptime: cfg.as_ref().map(|c| c.show_uptime).unwrap_or(true),
        processes: cfg.as_ref().map(|c| c.show_processes).unwrap_or(true),

        cpu_name: cfg.as_ref().map(|c| c.show_cpu_name).unwrap_or(true),
        cpu_freq: cfg.as_ref().map(|c| c.show_cpu_freq).unwrap_or(true),
        cpu_usage: cfg.as_ref().map(|c| c.show_cpu_usage).unwrap_or(true),
        cpu_temp: cfg.as_ref().map(|c| c.show_cpu_temp).unwrap_or(true),
        cpu_cores: cfg.as_ref().map(|c| c.show_cpu_cores).unwrap_or(true),
        cpu_arch: cfg.as_ref().map(|c| c.show_cpu_arch).unwrap_or(true),

        gpu_name: cfg.as_ref().map(|c| c.show_gpu_name).unwrap_or(true),
        gpu_temp: cfg.as_ref().map(|c| c.show_gpu_temp).unwrap_or(true),
        gpu_vram: cfg.as_ref().map(|c| c.show_gpu_vram).unwrap_or(true),

        ram: cfg.as_ref().map(|c| c.show_ram).unwrap_or(true),
        swap: cfg.as_ref().map(|c| c.show_swap).unwrap_or(true),

        de: cfg.as_ref().map(|c| c.show_de).unwrap_or(true),
        wm: cfg.as_ref().map(|c| c.show_wm).unwrap_or(true),
        terminal: cfg.as_ref().map(|c| c.show_terminal).unwrap_or(true),
        shell: cfg.as_ref().map(|c| c.show_shell).unwrap_or(true),
        local_ip: cfg.as_ref().map(|c| c.show_local_ip).unwrap_or(true),
        battery: cfg.as_ref().map(|c| c.show_battery).unwrap_or(true),
        locale_time: cfg.as_ref().map(|c| c.show_locale_time).unwrap_or(true),
        fetch_info: decide_show(
            args.hide_fetch_info,
            cfg.as_ref().map(|c| c.show_fetch_info),
            config_stronger,
        ),

        show_disks: cfg.as_ref().map(|c| c.show_disks).unwrap_or(true),
    };

    let info = SystemInfo::collect(opts);
    print::render(&info);

    #[cfg(windows)]
    {
        use std::io::IsTerminal;
        if std::io::stdin().is_terminal() && !args.no_pause {
            let mut dummy = String::new();
            let _ = std::io::stdin().read_line(&mut dummy);
        }
    }
}

fn decide_show(flag_hides: bool, config: Option<bool>, config_stronger: bool) -> bool {
    if config_stronger {
        config.unwrap_or(true)
    } else if flag_hides {
        false
    } else {
        config.unwrap_or(true)
    }
}

fn decide_enable(flag_enables: bool, config: Option<bool>, config_stronger: bool) -> bool {
    if config_stronger {
        config.unwrap_or(false)
    } else if flag_enables {
        true
    } else {
        config.unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decide_show() {
        // Flag hides = true -> false
        assert!(!decide_show(true, Some(true), false));
        // Flag hides = false -> config value
        assert!(decide_show(false, Some(true), false));
        assert!(!decide_show(false, Some(false), false));
        // Config stronger = true -> config wins
        assert!(decide_show(true, Some(true), true));
        assert!(!decide_show(false, Some(false), true));
    }

    #[test]
    fn test_decide_enable() {
        // Flag enables = true -> true
        assert!(decide_enable(true, Some(false), false));
        // Flag enables = false -> config value
        assert!(!decide_enable(false, Some(false), false));
        assert!(decide_enable(false, Some(true), false));
        // Config stronger = true -> config wins
        assert!(!decide_enable(true, Some(false), true));
        assert!(decide_enable(false, Some(true), true));
    }
}



