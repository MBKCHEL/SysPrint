use crate::sysinfo::combine::DisplayOptions;
use colored::Colorize;
use std::fmt::Write;
use sysinfo::System;
use crate::logos::parse_color;

// --- MEMORY INFO ---
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

    if opts.memory.swap {
        swap_info(sys, buf, c)
    }

    let total_ram = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_ram = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    let _ = writeln!(buf, "{}: {:.2} GB / {:.2} GB", parse_color("RAM", c), used_ram, total_ram);
}

pub fn swap_info(sys: &System, buf: &mut String, c: &str) {
    let total_swap = sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_swap = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;

    let _ = writeln!(buf, "{}: {:.2} GB / {:.2} GB", parse_color("Swap", c), used_swap, total_swap);
}