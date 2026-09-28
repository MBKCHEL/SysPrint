use crate::sysinfo::battery::battery_info;
use crate::sysinfo::combine::DisplayOptions;
use crate::sysinfo::desktop::{de_check, wm_check};
use crate::sysinfo::display::resolution_info;
use crate::sysinfo::network::local_ip;
use crate::sysinfo::terminal::{get_shell, terminal_info};
use chrono::Local;
use colored::ColoredString;
use std::fmt::Write;

pub fn other_info(
    opts: &DisplayOptions,
    buf: &mut String,
    c: fn(&str) -> ColoredString,
) {
    if !opts.other {
        return;
    }

    if !opts.compact_mode {
        let _ = writeln!(buf, "{}", opts.format_header("--- Other INFO ---", c));
    }

    if opts.de {
        de_check(buf, c);
    }
    if opts.fetch_info {
        sysprint_info(opts, buf, c);
    }
    if opts.compact_mode {
        return;
    }

    if opts.wm {
        wm_check(buf, c);
    }
    if opts.terminal {
        terminal_info(buf, c);
    }
    if opts.shell {
        get_shell(opts.fast_mode, buf, c);
    }
    if opts.resolution {
        resolution_info(buf, c);
    }
    if opts.local_ip {
        local_ip(opts.fast_mode, buf, c);
    }
    if opts.battery {
        battery_info(opts, buf, c);
    }
    if opts.locale_time {
        system_time(buf, c);
    }
}

fn sysprint_info(
    opts: &DisplayOptions,
    buf: &mut String,
    c: fn(&str) -> ColoredString,
) {
    let time_placeholder = if opts.show_time {
        "__SYSPRINT_TIME__"
    } else {
        ""
    };

    let _ = writeln!(
        buf,
        "{}: SysPrint v{}{}",
        c("Fetch"),
        env!("CARGO_PKG_VERSION"),
        time_placeholder
    );
}

fn system_time(buf: &mut String, c: fn(&str) -> ColoredString) {
    let now = Local::now();
    let _ = writeln!(buf, "{}: {}", c("Locale Time"), now.format("%H:%M"));
}
