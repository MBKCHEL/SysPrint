use sysinfo::System;
use crate::sysinfo::gpu::get_gpu_info;
use crate::sysinfo::memory::memory_info;
use crate::sysinfo::system::system_info;
use crate::sysinfo::cpu::cpu_info;
use crate::sysinfo::other::other_info;
use crate::sysinfo::disks::disk_info;

#[derive(Clone, Copy)]
pub struct DisplayOptions {
    pub system: bool,
    pub cpu: bool,
    pub memory: bool,
    pub disks: bool,
    pub other: bool,
    pub gpu: bool,
    pub mini_mode: bool,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            system: true,
            cpu: true,
            memory: true,
            disks: true,
            other: true,
            gpu: true,
            mini_mode: false,
        }
    }
}

pub struct SystemInfo {
    pub buffer: String,
    pub mini_mode: bool,
}

impl SystemInfo {
    pub fn collect(opts: DisplayOptions) -> Self {
        let _sys = System::new_all();

        let mut buffer = String::with_capacity(2048);

        let (_, _, c) = crate::logos::get_logo(opts.mini_mode);

        if opts.system {
            system_info(&opts, &mut buffer, c);
        }
        if opts.cpu {
            cpu_info(&opts, &mut buffer, &_sys, c);
        }
        if opts.gpu {
            get_gpu_info(&opts, &mut buffer, c);
        }
        if opts.memory {
            memory_info(&opts, &mut buffer, &_sys, c);
        }
        if opts.other {
            other_info(&opts, &mut buffer, c);
        }
        if opts.disks {
            disk_info(&opts, &mut buffer, c);
        }

        Self {
            buffer,
            mini_mode: opts.mini_mode,
        }
    }
}