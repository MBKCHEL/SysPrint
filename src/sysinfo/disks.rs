use colored::Colorize;
use sysinfo::{Disks};
use std::fmt::Write;
use crate::logos::parse_color;
use crate::sysinfo::combine::DisplayOptions;

// --- DISKS INFO ---
pub fn disk_info(opts: &DisplayOptions, buf: &mut String, c: &str) {
    if !opts.disks.enabled {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", "--- Disks INFO ---".bold().cyan());
    }

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total_gb = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_gb = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = total_gb - available_gb;

        let mount_point = disk.mount_point().to_string_lossy();
        let _ = writeln!(buf, "{}: {:.2} GB / {:.2} GB ({})", parse_color(&mount_point, c), used_gb, total_gb, disk.file_system().to_string_lossy());
    }
}