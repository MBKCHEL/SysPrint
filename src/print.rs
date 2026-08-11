//! Display system information.
use crate::logos;
use crate::sysinfo::combine::SystemInfo;


/// Renders the collected system info next to the OS ASCII logo.
pub fn render(info: &SystemInfo) {
    let lines = info.all_lines();

    let (logo, logo_padding) = logos::get_logo();
    let max_lines = std::cmp::max(logo.len(), lines.len());

    for i in 0..max_lines {
        if i < logo.len() {
            let logo_line = &logo[i];

            print!("{}", logo_line);

            let raw_len = strip_ansi_escapes::strip_str(logo_line.to_string())
                .chars()
                .count();

            let pad = if logo_padding > raw_len {
                logo_padding - raw_len + 3
            } else {
                3
            };
            print!("{}", " ".repeat(pad));
        } else {
            print!("{}", " ".repeat(logo_padding + 3));
        }

        let info_line = lines.get(i).unwrap_or(&String::new()).clone();
        println!("{}", info_line);
    }
}
