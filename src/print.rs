use std::io::{self, BufWriter, Write};
use crate::logos;
use crate::sysinfo::combine::SystemInfo;

pub fn render(info: &SystemInfo) {
    let (logo, logo_padding, _) = logos::get_logo( &info.logo, &info.logo_color, &info.custom_logo_path);

    let stdout = io::stdout();

    let mut handle = BufWriter::with_capacity(2048, stdout.lock());

    let mut info_lines = info.buffer.lines();
    let mut i = 0;

    loop {
        let logo_line = logo.get(i);
        let info_line = info_lines.next();

        if logo_line.is_none() && info_line.is_none() {
            break;
        }

        if let Some(logo_line) = logo_line {
            let pad = if logo_padding > logo_line.width {
                logo_padding - logo_line.width + 3
            } else {
                3
            };
            let _ = write!(handle, "{}{}", logo_line.text, " ".repeat(pad));
        } else {
            let _ = write!(handle, "{}", " ".repeat(logo_padding + 3));
        }

        let line = info_line.unwrap_or("");
        let _ = writeln!(handle, "{}", line);

        i += 1;
    }
}