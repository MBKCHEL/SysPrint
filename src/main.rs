mod logos;
mod parser;
mod print;
mod sysinfo;

use clap::Parser;
use parser::Arguments;
use sysinfo::{DisplayOptions, SystemInfo};

fn main() {
    let args = Arguments::parse();

    let opts = DisplayOptions {
        system: args.show_system_info,
        cpu: args.show_cpu_info,
        memory: args.show_memory_info,
        disks: args.show_disks_info,
        other: args.show_other_info,
        gpu: args.show_gpu_info,
    };

    let info = SystemInfo::collect(opts);
    print::render(&info);

    #[cfg(windows)]
    {
        use std::io::stdin;
        let mut dummy = String::new();
        let _ = stdin().read_line(&mut dummy);
    }
}