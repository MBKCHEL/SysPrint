use crate::sysinfo::combine::DisplayOptions;
use colored::ColoredString;
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
        let _ = writeln!(buf, "{}", opts.format_header("--- Memory INFO ---", c));
    }
    if opts.ram {
        let total_ram = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_ram = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let bar = if opts.progress_bars && total_ram > 0.0 {
            format!("{} ", crate::sysinfo::combine::make_bar((used_ram / total_ram) * 100.0, 10, c))
        } else {
            String::new()
        };
        let _ = writeln!(
            buf,
            "{}: {}{:.2} GB / {:.2} GB",
            c("RAM"),
            bar,
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
            let bar = if opts.progress_bars {
                format!("{} ", crate::sysinfo::combine::make_bar((used_swap / total_swap) * 100.0, 10, c))
            } else {
                String::new()
            };
            let _ = writeln!(
                buf,
                "{}: {}{:.2} GB / {:.2} GB",
                c("Swap"),
                bar,
                used_swap,
                total_swap
            );
        }
    }
}

