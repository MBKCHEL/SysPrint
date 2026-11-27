use crate::config::{
    CpuConfig, DisksConfig, GpuConfig, MemoryConfig, OrderConfig, OtherConfig, SystemConfig,
};
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
    pub compact_mode: bool,
    pub fast_mode: bool,
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
            compact_mode: false,
            fast_mode: false,
            show_sysprint_start_time: true,
        }
    }
}

pub struct SystemInfo {
    pub buffer: String,
    pub logo: String,
    pub logo_color: String,
    pub custom_logo_path: String,
}

impl SystemInfo {
    pub fn collect(mut opts: DisplayOptions) -> Self {
        let start_time = std::time::Instant::now();
        let mut buffer = String::with_capacity(2048);

        if opts.order.order.is_empty() {
            opts.order.order = vec![
                "system".into(),
                "cpu".into(),
                "gpu".into(),
                "memory".into(),
                "other".into(),
                "disks".into(),
            ];
        }

        if opts.system.order_system.is_empty() {
            opts.system.order_system = vec![
                "os-name".into(),
                "os-version".into(),
                "kernel".into(),
                "init".into(),
                "user".into(),
                "uptime".into(),
            ];
        }

        if opts.cpu.order_cpu.is_empty() {
            opts.cpu.order_cpu = vec![
                "cpu-name".into(),
                "cpu-ghz".into(),
                "cpu-usage".into(),
                "cpu-temp".into(),
                "cpu-cores".into(),
                "cpu-threads".into(),
            ];
        }

        if opts.memory.order_memory.is_empty() {
            opts.memory.order_memory = vec!["ram".into(), "swap".into()];
        }

        if opts.other.order_other.is_empty() {
            opts.other.order_other = vec![
                "de".into(),
                "fetch-info".into(),
                "wm".into(),
                "terminal".into(),
                "shell".into(),
                "battery".into(),
                "locale-time".into(),
            ]
        }

        let _sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        )
        .unwrap();

        let (_, _, c) = logos::get_logo(
            &opts.logo,
            &opts.logo_color,
            &opts.custom_logo_path,
        );

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
            };
        }

        let mut parts: Vec<String> = vec![String::new(); opts.order.order.len()];

        std::thread::scope(|s| {
            for (item, out) in opts.order.order.iter().zip(parts.iter_mut()) {
                let opts = &opts;
                let sys = &_sys;
                let c = &c;

                s.spawn(move || match item.as_str() {
                    "system" => {
                        if opts.system.enabled {
                            system_info(opts, out, c);
                        }
                    }
                    "cpu" => {
                        if opts.cpu.enabled {
                            cpu_info(opts, out, sys, c);
                        }
                    }
                    "gpu" => {
                        if opts.gpu.enabled {
                            get_gpu_info(opts, out, opts.fast_mode, c);
                        }
                    }
                    "memory" => {
                        if opts.memory.enabled {
                            memory_info(opts, out, sys, c);
                        }
                    }
                    "other" => {
                        if opts.other.enabled {
                            other_info(opts, out, c, start_time);
                        }
                    }
                    "disks" => {
                        if opts.disks.enabled {
                            disk_info(opts, out, c);
                        }
                    }
                    _ => {}
                });
            }
        });

        for part in parts {
            buffer.push_str(&part);
        }

        Self {
            buffer,
            logo: opts.logo,
            logo_color: opts.logo_color,
            custom_logo_path: opts.custom_logo_path,
        }
    }
}
