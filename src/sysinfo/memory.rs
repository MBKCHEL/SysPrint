use crate::sysinfo::combine::DisplayOptions;
use colored::ColoredString;
use colored::Colorize;
use std::fmt::Write;
use sysinfo::System;

// --- MEMORY INFO ---
pub fn memory_info(
    opts: &DisplayOptions,
    buf: &mut String,
    sys: &System,
    c: fn(&str) -> ColoredString,
) {
    if !opts.memory {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", "--- Memory INFO ---".bold().cyan());
    }
    if opts.ram {
        let total_ram = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_ram = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let _ = writeln!(
            buf,
            "{}: {:.2} GB / {:.2} GB",
            c("RAM"),
            used_ram,
            total_ram
        );
    }

    if opts.compact_mode {
        return;
    }

    if opts.swap {
        let total_swap = sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_swap = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
        if total_swap > 0.0 {
            let _ = writeln!(
                buf,
                "{}: {:.2} GB / {:.2} GB",
                c("Swap"),
                used_swap,
                total_swap
            );
        }
    }
}

