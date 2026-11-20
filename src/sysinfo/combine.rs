use crate::config::{CpuConfig, DisksConfig, GpuConfig, MemoryConfig, OrderConfig, OtherConfig, SystemConfig};
use crate::logos;
use crate::sysinfo::cpu::cpu_info;
use crate::sysinfo::disks::disk_info;
use crate::sysinfo::gpu::get_gpu_info;
use crate::sysinfo::memory::memory_info;
use crate::sysinfo::other::other_info;
use crate::sysinfo::system::system_info;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[derive(Clone)]
pub struct DisplayOptions {
    pub system: SystemConfig,
    pub order: OrderConfig,
    pub cpu: CpuConfig,
    pub gpu: GpuConfig,
    pub memory: MemoryConfig,
    pub other: OtherConfig,
    pub disks: DisksConfig,
    pub logo: String,
    pub logo_color: String,
    pub custom_logo_path: String,
    pub mini_logo_mode: bool,
    pub compact_mode: bool,
    pub fast_mode: bool,
    pub hide_fetch_info: bool,
    pub show_sysprint_start_time: bool,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            order: OrderConfig::default(),
            system: SystemConfig::default(),
            cpu: CpuConfig::default(),
            gpu: GpuConfig::default(),
            memory: MemoryConfig::default(),
            other: OtherConfig::default(),
            disks: DisksConfig::default(),
            logo: String::from("default"),
            logo_color: String::from("default"),
            custom_logo_path: String::from("default"),
            mini_logo_mode: false,
            compact_mode: false,
            fast_mode: false,
            hide_fetch_info: false,
            show_sysprint_start_time: true,
        }
    }
}

pub struct SystemInfo {
    pub buffer: String,
    pub logo: String,
    pub logo_color: String,
    pub custom_logo_path: String,
    pub mini_logo_mode: bool,
}

impl SystemInfo {
    pub fn collect(mut opts: DisplayOptions) -> Self {
        let start_time = std::time::Instant::now();
        let mut buffer = String::with_capacity(2048);

        if opts.order.order.is_empty() {
            opts.order.order = vec![
                "system".into(), "cpu".into(), "gpu".into(),
                "memory".into(), "other".into(), "disks".into()
            ];
        }

        let _sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        ).unwrap();

        let (_, _, c) = logos::get_logo(opts.mini_logo_mode, &opts.logo, &opts.logo_color, &opts.custom_logo_path);

        if opts.compact_mode {
            system_info(&opts, &mut buffer, &c);
            cpu_info(&opts, &mut buffer, &_sys, &c);
            get_gpu_info(&opts, &mut buffer, opts.fast_mode, &c);
            memory_info(&opts, &mut buffer, &_sys, &c);
            other_info(&opts, &mut buffer, &c, start_time);

            return Self {
                buffer,
                logo: opts.logo,
                logo_color: opts.logo_color,
                custom_logo_path: opts.custom_logo_path,
                mini_logo_mode: opts.mini_logo_mode,
            };
        }


        for item in &opts.order.order {
            match item.as_str() {
                "system" => if opts.system.enabled {
                    system_info(&opts, &mut buffer, &c);
                },
                "cpu" => if opts.cpu.enabled {
                    cpu_info(&opts, &mut buffer, &_sys, &c);
                },
                "gpu" => if opts.gpu.enabled {
                    get_gpu_info(&opts, &mut buffer, opts.fast_mode, &c);
                },
                "memory" => if opts.memory.enabled {
                        memory_info(&opts, &mut buffer, &_sys, &c);
                },
                "other" => if opts.other.enabled {
                    other_info(&opts, &mut buffer, &c, start_time);
                },
                "disks" => if opts.disks.enabled {
                    disk_info(&opts, &mut buffer, &c);
                },
                _ => {}
            }
        }


        Self {
            buffer,
            logo: opts.logo,
            logo_color: opts.logo_color,
            custom_logo_path: opts.custom_logo_path,
            mini_logo_mode: opts.mini_logo_mode,
        }
    }
}
