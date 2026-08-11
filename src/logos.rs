use sysinfo::System;
use colored::*;

pub fn get_logo() -> (Vec<ColoredString>, usize) {
    let os_name = System::name().unwrap_or_default().to_lowercase();

    
    let max_width = raw_logo.lines().map(|l| l.chars().count()).max().unwrap_or(0);

    let logo_lines = raw_logo
        .lines()
        .map(|line| {
            let char_count = line.chars().count();
            let padding = " ".repeat(max_width.saturating_sub(char_count));
            color_func(&format!("{line}{padding}"))
        })
        .collect();

    (logo_lines, max_width)
}
