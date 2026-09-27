use crate::sysinfo::cpu::cpu_info;
use crate::sysinfo::disks::disk_info;
use crate::sysinfo::gpu::get_gpu_info;
use crate::sysinfo::memory::memory_info;
use crate::sysinfo::other::other_info;
use crate::sysinfo::system::system_info;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

#[derive(Clone)]
pub struct DisplayOptions {
    pub custom_logo: Option<String>,
    pub mini_logo_mode: bool,
    pub compact_mode: bool,
    pub fast_mode: bool,

    // Master section toggles
    pub system: bool,
    pub cpu: bool,
    pub gpu: bool,
    pub memory: bool,
    pub other: bool,
    pub disks: bool,

    // Granular System
    pub os: bool,
    pub kernel: bool,
    pub os_version: bool,
    pub init: bool,
    pub host: bool,
    pub user: bool,
    pub uptime: bool,
    pub processes: bool,

    // Granular CPU
    pub cpu_name: bool,
    pub cpu_freq: bool,
    pub cpu_usage: bool,
    pub cpu_temp: bool,
    pub cpu_cores: bool,
    pub cpu_arch: bool,

    // Granular GPU
    pub gpu_name: bool,
    pub gpu_temp: bool,
    pub gpu_vram: bool,

    // Granular Memory
    pub ram: bool,
    pub swap: bool,

    // Granular Other
    pub de: bool,
    pub wm: bool,
    pub terminal: bool,
    pub shell: bool,
    pub resolution: bool,
    pub local_ip: bool,
    pub battery: bool,
    pub locale_time: bool,
    pub fetch_info: bool,

    // Granular Disks
    pub show_disks: bool,
    pub all_disks: bool,

    // Colors
    pub accent_color: Option<String>,
    pub header_color: Option<String>,

    // Progress bars
    pub progress_bars: bool,

    // Modes
    pub no_logo: bool,
    pub show_time: bool,
}

pub fn make_bar(percent: f64, width: usize, c: fn(&str) -> colored::ColoredString) -> String {
    let clamped = percent.clamp(0.0, 100.0);
    let filled = ((clamped / 100.0) * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    let filled_str = "█".repeat(filled);
    let empty_str = "░".repeat(empty);
    format!("[{}{}]", c(&filled_str), empty_str)
}

impl DisplayOptions {
    pub fn format_header(&self, title: &str, default_c: fn(&str) -> colored::ColoredString) -> colored::ColoredString {
        use colored::Colorize;
        if let Some(ref hc) = self.header_color {
            let lower = hc.trim().to_lowercase();
            if lower == "accent" {
                return default_c(title);
            }
            if lower != "auto"
                && let Some(cf) = crate::logos::resolve_color(&lower)
            {
                return cf(title);
            }
        }
        title.bold().cyan()
    }
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            custom_logo: None,
            mini_logo_mode: false,
            compact_mode: false,
            fast_mode: false,
            progress_bars: false,
            no_logo: false,
            show_time: false,

            system: true,
            cpu: true,
            gpu: true,
            memory: true,
            other: true,
            disks: true,

            os: true,
            kernel: true,
            os_version: true,
            init: true,
            host: true,
            user: true,
            uptime: true,
            processes: true,

            cpu_name: true,
            cpu_freq: true,
            cpu_usage: false,
            cpu_temp: true,
            cpu_cores: true,
            cpu_arch: true,

            gpu_name: true,
            gpu_temp: true,
            gpu_vram: true,

            ram: true,
            swap: true,

            de: true,
            wm: true,
            terminal: true,
            shell: true,
            resolution: true,
            local_ip: true,
            battery: true,
            locale_time: true,
            fetch_info: true,

            show_disks: true,
            all_disks: false,

            accent_color: None,
            header_color: None,
        }
    }
}

pub struct SystemInfo {
    pub buffer: String,
    pub logo: Vec<colored::ColoredString>,
    pub logo_padding: usize,
}

impl SystemInfo {
    pub fn collect(opts: DisplayOptions) -> Self {
        let mut buffer = String::with_capacity(2048);
        let (logo, logo_padding, mut c) = if opts.no_logo {
            let (_, _, color_fn) = crate::logos::get_logo(opts.mini_logo_mode, opts.custom_logo.as_deref());
            (Vec::new(), 0, color_fn)
        } else {
            crate::logos::get_logo(opts.mini_logo_mode, opts.custom_logo.as_deref())
        };

        if let Some(ref ac) = opts.accent_color {
            let lower = ac.trim().to_lowercase();
            if lower != "auto"
                && let Some(override_c) = crate::logos::resolve_color(&lower)
            {
                c = override_c;
            }
        }

        let start = std::time::Instant::now();
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(ProcessRefreshKind::nothing()),
        );

        if opts.system {
            system_info(&opts, &mut buffer, &sys, c);
        }
        if opts.cpu {
            if opts.cpu_usage && !opts.fast_mode && !opts.compact_mode {
                let elapsed = start.elapsed();
                if elapsed < sysinfo::MINIMUM_CPU_UPDATE_INTERVAL {
                    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL - elapsed);
                }
                sys.refresh_cpu_usage();
            }
            cpu_info(&opts, &mut buffer, &sys, c);
        }
        if opts.gpu {
            get_gpu_info(&opts, &mut buffer, opts.fast_mode, c);
        }
        if opts.memory {
            memory_info(&opts, &mut buffer, &sys, c);
        }
        if opts.other {
            other_info(&opts, &mut buffer, c);
        }
        if opts.disks {
            disk_info(&opts, &mut buffer, c);
        }

        if opts.show_time {
            let elapsed_ms = format!(" ({:.1}ms)", start.elapsed().as_secs_f64() * 1000.0);
            buffer = buffer.replace("__SYSPRINT_TIME__", &elapsed_ms);
        }

        Self {
            buffer,
            logo,
            logo_padding,
        }
    }
}
