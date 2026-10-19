use crate::sysinfo::combine::DisplayOptions;
use colored::Colorize;
use std::fmt::Write;
use sysinfo::System;
use crate::logos::parse_color;

pub fn memory_info(
    opts: &DisplayOptions,
    buf: &mut String,
    sys: &System,
    c: &str
) {
    if !opts.memory.enabled {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", "--- Memory INFO ---".bold().cyan());
    }

    ram_info(opts, sys, buf, c);

    if opts.memory.swap {
        swap_info(opts, sys, buf, c);
    }
}

pub fn ram_info(opts: &DisplayOptions, sys: &System, buf: &mut String, c: &str) {
    let total_ram = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_ram = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    let (label, val_str) = match (opts.memory.ram_used, opts.memory.ram_total) {
        (true, true)   => ("RAM", format!("{:.2} GB / {:.2} GB", used_ram, total_ram)),
        (true, false)  => ("Used RAM", format!("{:.2} GB", used_ram)),
        (false, true)  => ("Total RAM", format!("{:.2} GB", total_ram)),
        (false, false) => return,
    };

    let _ = writeln!(buf, "{}: {}", parse_color(label, c), val_str);
}

pub fn swap_info(opts: &DisplayOptions, sys: &System, buf: &mut String, c: &str) {
    let total_swap = sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_swap = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;

    if total_swap <= 0.001 {
        return;
    }

    let (label, val_str) = match (opts.memory.swap_used, opts.memory.swap_total) {
        (true, true)   => ("Swap", format!("{:.2} GB / {:.2} GB", used_swap, total_swap)),
        (true, false)  => ("Used Swap", format!("{:.2} GB", used_swap)),
        (false, true)  => ("Total Swap", format!("{:.2} GB", total_swap)),
        (false, false) => return,
    };

    let _ = writeln!(buf, "{}: {}", parse_color(label, c), val_str);
}