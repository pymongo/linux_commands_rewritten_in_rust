fn main() {}

fn get_default_route_network_interface_by_ip_route() -> String {
    let output = std::process::Command::new("ip")
        .arg("route")
        .arg("show")
        .arg("default")
        .output()
        .unwrap();
    // output e.g. "default via 192.168.18.1 dev wlp4s0 proto dhcp metric 600 \n"
    let output = unsafe { String::from_utf8_unchecked(output.stdout) };

    // `ip route show default` parser
    // `split()` is similar to `libc::strtok()`
    output.split_whitespace().nth(4).unwrap().to_string()
}

/// get mac/physics address by network interface
fn get_mac_addr_by_network_interface(network_interface: String) -> String {
    std::fs::read_to_string(format!("/sys/class/net/{}/address", network_interface))
        .unwrap_or_default()
        .trim_end()
        .to_string()
}
