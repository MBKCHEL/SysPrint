//
use sysinfo::System;
use crate::sysinfo::gpu::get_gpu_info;
use crate::sysinfo::memory::memory_info;
use crate::sysinfo::system::system_info;
use crate::sysinfo::cpu::cpu_info;
use crate::sysinfo::other::other_info;
use crate::sysinfo::disks::disk_info;


/// Which sections of the report should be collected.
#[derive(Clone, Copy)]
pub struct DisplayOptions {
    pub system: bool,
    pub cpu: bool,
    pub memory: bool,
    pub disks: bool,
    pub other: bool,
    pub gpu: bool,
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
        }
    }
}

pub struct SystemInfo {
    pub lines: Vec<String>,
    pub memory: Vec<String>,
    pub gpu: Vec<String>,
    pub system: Vec<String>,
    pub cpu: Vec<String>,
    pub other: Vec<String>,
    pub disk: Vec<String>,
}

impl SystemInfo {
    /// Collect all system information into a renderable form.
    pub fn collect(opts: DisplayOptions) -> Self {
        let _sys = System::new_all();

        let lines: Vec<String> = Vec::new();

        let mut memory = Vec::new();

        let mut other = Vec::new();

        let mut system = Vec::new();

        let mut cpu = Vec::new();

        let mut disk = Vec::new();

        // Memory info
        if opts.memory {
            memory_info(&opts, &mut memory, &_sys);
        }

        // System info
        if opts.system {
            system_info(&opts,  &mut system);
        }

        //CPU info
        if opts.cpu {
            cpu_info(&opts,  &mut cpu, &_sys);
        }

        //Other info
        if opts.other {
            other_info(&opts,  &mut other);
        }

        //Disk info
        if opts.disks {
            disk_info(&opts,  &mut disk);
        }

        // GPU info
        let gpu = if opts.gpu {
            get_gpu_info(&opts)
        } else {
            Vec::new()
        };


        Self {
            lines,
            memory,
            gpu,
            system,
            cpu,
            other,
            disk
        }
    }


    pub fn all_lines(&self) -> Vec<String> {
        let mut all = self.lines.clone();
        all.extend(self.system.clone());
        all.extend(self.cpu.clone());
        all.extend(self.gpu.clone());
        all.extend(self.memory.clone());
        all.extend(self.other.clone());
        all.extend(self.disk.clone());
        all
    }
}
