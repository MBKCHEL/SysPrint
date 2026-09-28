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

pub fn get_fast_local_ip() -> Option<String> {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("1.1.1.1:80").ok()?;
    let local_addr = socket.local_addr().ok()?;
    let ip = local_addr.ip();
    if ip.is_loopback() || ip.is_unspecified() {
        None
    } else {
        Some(ip.to_string())
    }
}

pub fn local_ip(fast_mode: bool, buf: &mut String, c: fn(&str) -> ColoredString) {
    let ip_str = if fast_mode {
        get_fast_local_ip()
    } else {
        get_local_ip_string()
    };

    if let Some(ip) = ip_str {
        let _ = writeln!(buf, "{}: {}", c("Local IP"), ip);
    }
}
