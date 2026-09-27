use colored::Colorize;
use sysinfo::{Disks};
use colored::{ColoredString};
use std::fmt::Write;
use crate::sysinfo::combine::DisplayOptions;

// --- DISKS INFO ---
pub fn disk_info(opts: &DisplayOptions, buf: &mut String, c :fn(&str) -> ColoredString) {
    if !opts.disks || !opts.show_disks || opts.fast_mode {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", "--- Disks INFO ---".bold().cyan());
    }

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let mount_point = disk.mount_point().to_string_lossy();
        if opts.compact_mode && mount_point != "/" && !mount_point.starts_with("C:") {
            continue;
        }

        let total_gb = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_gb = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = total_gb - available_gb;

        let _ = writeln!(buf,
            "{}: {:.2} GB / {:.2} GB ({})",
                         c(&mount_point),
                         used_gb,
                         total_gb,
                         disk.file_system()
                             .to_string_lossy());
    }
}