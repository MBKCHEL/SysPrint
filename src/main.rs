mod logos;
mod parser;
mod print;
mod sysinfo;

use clap::Parser;
use parser::Arguments;
use sysinfo::SystemInfo;

fn main() {
    let args = Arguments::parse();

    let info = SystemInfo::collect(args.show_gpu_info);
    print::render(&info);

    #[cfg(windows)]
    {
        use std::io::stdin;
        let mut dummy = String::new();
        let _ = stdin().read_line(&mut dummy);
    }
}
