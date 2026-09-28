use serde::Serialize;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

use crate::sysinfo::combine::DisplayOptions;
use crate::sysinfo::display::get_resolution;

#[derive(Serialize)]
pub struct SysPrintJson {
    pub fetch: FetchJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<CpuJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<Vec<GpuJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<OtherJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<DiskJson>>,
}

#[derive(Serialize)]
pub struct FetchJson {
    pub name: &'static str,
    pub version: &'static str,
}

#[derive(Serialize)]
pub struct SystemJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_avg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_processes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zombie_processes: Option<usize>,
}

#[derive(Serialize)]
pub struct CpuJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_ghz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_percent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_c: Option<f32>,
}

#[derive(Serialize)]
pub struct GpuJson {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_c: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vram_used_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vram_total_bytes: Option<u64>,
}

#[derive(Serialize)]
pub struct MemoryJson {
    pub ram_used_bytes: u64,
    pub ram_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
}

#[derive(Serialize)]
pub struct OtherJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_time: Option<String>,
}

#[derive(Serialize)]
pub struct DiskJson {
    pub mount: String,
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub fs_type: String,
}

pub fn collect_json(opts: &DisplayOptions) -> String {
    let start = std::time::Instant::now();
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
            .with_processes(ProcessRefreshKind::nothing()),
    );

    if opts.cpu && opts.cpu_usage && !opts.fast_mode {
        let elapsed = start.elapsed();
        if elapsed < sysinfo::MINIMUM_CPU_UPDATE_INTERVAL {
            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL - elapsed);
        }
        sys.refresh_cpu_usage();
    }

    let system = if opts.system {
        let proc_stats = if opts.processes {
            Some(crate::sysinfo::system::get_process_stats(&sys, opts.fast_mode))
        } else {
            None
        };
        Some(SystemJson {
            os: if opts.os { System::name() } else { None },
            kernel: if opts.kernel { System::kernel_version() } else { None },
            os_version: if opts.os_version { System::os_version() } else { None },
            init: if opts.init {
                Some(crate::sysinfo::system::get_init_system())
            } else {
                None
            },
            host: if opts.host { System::host_name() } else { None },
            user: if opts.user {
                std::env::var("USER").or_else(|_| std::env::var("USERNAME")).ok()
            } else {
                None
            },
            uptime: if opts.uptime {
                let sec = System::uptime();
                Some(format!("{}h {}m", sec / 3600, (sec % 3600) / 60))
            } else {
                None
            },
            load_avg: if opts.load_avg {
                let load = System::load_average();
                if load.one > 0.0 || load.five > 0.0 || load.fifteen > 0.0 {
                    Some(format!("{:.2}, {:.2}, {:.2}", load.one, load.five, load.fifteen))
                } else {
                    None
                }
            } else {
                None
            },
            processes: proc_stats.as_ref().and_then(|s| if s.total > 0 { Some(s.total) } else { None }),
            running_processes: proc_stats.as_ref().and_then(|s| if s.running > 0 || s.zombie > 0 { Some(s.running) } else { None }),
            zombie_processes: proc_stats.as_ref().and_then(|s| if s.running > 0 || s.zombie > 0 { Some(s.zombie) } else { None }),
        })
    } else {
        None
    };

    let cpu = if opts.cpu {
        let cpu_obj = sys.cpus().first();
        Some(CpuJson {
            name: if opts.cpu_name {
                cpu_obj.map(|c| c.brand().trim().to_string())
            } else {
                None
            },
            frequency_ghz: if opts.cpu_freq {
                cpu_obj.map(|c| (c.frequency() as f64) / 1000.0)
            } else {
                None
            },
            cores: if opts.cpu_cores {
                sysinfo::System::physical_core_count()
            } else {
                None
            },
            threads: if opts.cpu_cores {
                Some(sys.cpus().len())
            } else {
                None
            },
            arch: if opts.cpu_arch {
                Some(std::env::consts::ARCH.to_string())
            } else {
                None
            },
            usage_percent: if opts.cpu_usage {
                Some(sys.global_cpu_usage())
            } else {
                None
            },
            temp_c: if opts.cpu_temp {
                crate::sysinfo::cpu::get_cpu_temp()
            } else {
                None
            },
        })
    } else {
        None
    };

    let gpu = if opts.gpu {
        crate::sysinfo::gpu::get_primary_gpu_name().map(|name| {
            vec![GpuJson {
                name,
                temp_c: None,
                vram_used_bytes: None,
                vram_total_bytes: None,
            }]
        })
    } else {
        None
    };

    let memory = if opts.memory {
        Some(MemoryJson {
            ram_used_bytes: sys.used_memory(),
            ram_total_bytes: sys.total_memory(),
            swap_used_bytes: sys.used_swap(),
            swap_total_bytes: sys.total_swap(),
        })
    } else {
        None
    };

    let other = if opts.other {
        Some(OtherJson {
            de: if opts.de {
                Some(crate::sysinfo::desktop::get_de())
            } else {
                None
            },
            wm: if opts.wm {
                Some(crate::sysinfo::desktop::get_wm())
            } else {
                None
            },
            terminal: if opts.terminal {
                Some(crate::sysinfo::terminal::get_terminal())
            } else {
                None
            },
            shell: if opts.shell {
                Some(crate::sysinfo::terminal::get_shell_string(opts.fast_mode))
            } else {
                None
            },
            resolution: if opts.resolution {
                get_resolution()
            } else {
                None
            },
            local_ip: if opts.local_ip {
                if opts.fast_mode {
                    crate::sysinfo::network::get_fast_local_ip()
                } else {
                    crate::sysinfo::network::get_local_ip_string()
                }
            } else {
                None
            },
            battery: if opts.battery {
                crate::sysinfo::battery::get_battery_raw().map(|(pct, status)| match status {
                    Some(s) => format!("{pct}% [{s}]"),
                    None => format!("{pct}%"),
                })
            } else {
                None
            },
            locale_time: if opts.locale_time {
                Some(chrono::Local::now().format("%H:%M").to_string())
            } else {
                None
            },
        })
    } else {
        None
    };

    let disks = if opts.disks {
        let disks_obj = Disks::new_with_refreshed_list();
        Some(
            disks_obj
                .iter()
                .filter(|d| opts.all_disks || crate::sysinfo::disks::is_physical_disk(d))
                .map(|d| DiskJson {
                    mount: d.mount_point().to_string_lossy().into_owned(),
                    used_bytes: d.total_space().saturating_sub(d.available_space()),
                    total_bytes: d.total_space(),
                    fs_type: d.file_system().to_string_lossy().into_owned(),
                })
                .collect(),
        )
    } else {
        None
    };

    let result = SysPrintJson {
        fetch: FetchJson {
            name: "SysPrint",
            version: env!("CARGO_PKG_VERSION"),
        },
        system,
        cpu,
        gpu,
        memory,
        other,
        disks,
    };

    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}
