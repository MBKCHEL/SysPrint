use colored::Colorize;
use sysinfo::{System};
use crate::sysinfo::combine::DisplayOptions;

// --- MEMORY INFO ---
pub fn memory_info<'a>(opts: &'a DisplayOptions, lines: &'a mut Vec<String>, sys: &'a System) -> &'a mut Vec<String> {
    opts.memory;
    lines.push(format!("{}", "--- Memory INFO ---".bold().cyan()));
    let total_ram = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_ram = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    lines.push(format!("{}: {:.2} GB / {:.2} GB", "RAM".bold(), used_ram, total_ram));
    lines
}
