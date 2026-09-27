use colored::ColoredString;
use std::fmt::Write;
use sysinfo::Networks;

pub fn local_ip(buf: &mut String, c: fn(&str) -> ColoredString) {
    let networks = Networks::new_with_refreshed_list();

    // Prefer non-virtual IPv4
    for (iface, data) in &networks {
        let iface_str = iface.as_str();
        if iface_str == "lo"
            || iface_str.starts_with("docker")
            || iface_str.starts_with("br-")
            || iface_str.starts_with("veth")
            || iface_str.starts_with("virbr")
        {
            continue;
        }

        for ip_net in data.ip_networks() {
            let ip = ip_net.addr;
            if ip.is_ipv4() && !ip.is_loopback() {
                let _ = writeln!(buf, "{}: {} ({})", c("Local IP"), ip, iface);
                return;
            }
        }
    }

    // Fallback: any non-loopback IP
    for (iface, data) in &networks {
        if iface == "lo" {
            continue;
        }
        for ip_net in data.ip_networks() {
            let ip = ip_net.addr;
            if !ip.is_loopback() {
                let _ = writeln!(buf, "{}: {} ({})", c("Local IP"), ip, iface);
                return;
            }
        }
    }
}
