use colored::ColoredString;
use std::fmt::Write;
use sysinfo::Networks;

pub fn get_local_ip_string() -> Option<String> {
    let networks = Networks::new_with_refreshed_list();
    let mut ifaces: Vec<(&String, &sysinfo::NetworkData)> = networks.iter().collect();
    ifaces.sort_by_key(|(name, _)| (*name).clone());

    // Prefer non-virtual IPv4
    for (iface, data) in &ifaces {
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
                return Some(format!("{} ({})", ip, iface));
            }
        }
    }

    // Fallback: any non-loopback IP
    for (iface, data) in &ifaces {
        if iface.as_str() == "lo" {
            continue;
        }
        for ip_net in data.ip_networks() {
            let ip = ip_net.addr;
            if !ip.is_loopback() {
                return Some(format!("{} ({})", ip, iface));
            }
        }
    }

    None
}

pub fn local_ip(buf: &mut String, c: fn(&str) -> ColoredString) {
    if let Some(ip_str) = get_local_ip_string() {
        let _ = writeln!(buf, "{}: {}", c("Local IP"), ip_str);
    }
}
