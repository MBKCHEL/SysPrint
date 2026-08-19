use std::io::{self, Write};
use crate::logos;
use crate::sysinfo::combine::SystemInfo;

pub fn render(info: &SystemInfo) {
    let lines = info.all_lines();
    let (logo, logo_padding) = logos::get_logo(info.mini_mode);
    let max_lines = std::cmp::max(logo.len(), lines.len());

    let raw_lens: Vec<usize> = logo
        .iter()
        .map(|line| strip_ansi_escapes::strip_str(&line.to_string()).chars().count())
        .collect();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for i in 0..max_lines {
        if i < logo.len() {
            let logo_line = &logo[i];
            let raw_len = raw_lens[i];

            let pad = if logo_padding > raw_len {
                logo_padding - raw_len + 3
            } else {
                3
            };

            let _ = write!(handle, "{}{}", logo_line, " ".repeat(pad));
        } else {
            let _ = write!(handle, "{}", " ".repeat(logo_padding + 3));
        }

        let info_line = lines.get(i).map(|s| s.as_str()).unwrap_or("");
        let _ = writeln!(handle, "{}", info_line);
    }
}