use colored::Colorize;
use sysinfo::{Disks};
use crate::sysinfo::combine::DisplayOptions;

pub fn disk_info(opts: &DisplayOptions, lines: &mut Vec<String>) {
    // --- DISKS INFO ---
    opts.disks;
    lines.push(format!("{}", "--- Disks INFO ---".bold().cyan()));
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total_gb = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_gb = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = total_gb - available_gb;

        let mount_point = disk.mount_point().to_string_lossy();
        lines.push(format!(
            "{}: {:.2} GB / {:.2} GB ({})",
            mount_point.bold(),
            used_gb,
            total_gb,
            disk.file_system().to_string_lossy()
        ));
    }
}