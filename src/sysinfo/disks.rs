use colored::ColoredString;
use std::fmt::Write;
use sysinfo::Disks;
use crate::sysinfo::combine::DisplayOptions;

pub fn is_physical_disk(disk: &sysinfo::Disk) -> bool {
    if disk.total_space() == 0 {
        return false;
    }

    let fs = disk.file_system().to_string_lossy().to_lowercase();
    if fs == "squashfs"
        || fs == "overlay"
        || fs == "tmpfs"
        || fs == "devtmpfs"
        || fs == "ramfs"
        || fs == "iso9660"
        || fs == "none"
    {
        return false;
    }

    let mount = disk.mount_point().to_string_lossy();
    if mount.starts_with("/snap")
        || mount.starts_with("/var/lib/docker")
        || mount.starts_with("/var/lib/flatpak")
        || mount.starts_with("/run/")
        || mount.starts_with("/sys/")
        || mount.starts_with("/proc/")
        || mount.starts_with("/dev/")
    {
        return false;
    }

    true
}

// --- DISKS INFO ---
pub fn disk_info(opts: &DisplayOptions, buf: &mut String, c :fn(&str) -> ColoredString) {
    if !opts.disks || !opts.show_disks || opts.fast_mode {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", opts.format_header("--- Disks INFO ---", c));
    }

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        if !opts.all_disks && !is_physical_disk(disk) {
            continue;
        }

        let mount_point = disk.mount_point().to_string_lossy();
        if opts.compact_mode && mount_point != "/" && !mount_point.starts_with("C:") {
            continue;
        }

        let total_gb = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_gb = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = total_gb - available_gb;
        let bar = if opts.progress_bars && total_gb > 0.0 {
            format!("{} ", crate::sysinfo::combine::make_bar((used_gb / total_gb) * 100.0, 10, c))
        } else {
            String::new()
        };

        let _ = writeln!(
            buf,
            "{}: {}{:.2} GB / {:.2} GB ({})",
            c(&mount_point),
            bar,
            used_gb,
            total_gb,
            disk.file_system().to_string_lossy()
        );
    }
}