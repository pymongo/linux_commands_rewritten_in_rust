use linux_programming::{file_system::parser::proc, syscall};
use std::net::Ipv4Addr;

/// get default router network interface's mac/physics address
/// use `ip route` or `route` command to get(route command require net-tools package)
/// ## Alternative
/// find first Iface which Gateway != 0 in `/proc/net/route`
fn main() {
    let route_default_network_interface = proc::net::route::default_route_network_interface();
    assert_eq!(
        route_default_network_interface,
        get_default_route_network_interface_by_ip_route()
    );
    dbg!(get_mac_addr_by_network_interface(
        route_default_network_interface
    ));
}

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

unsafe fn get_default_route_ip() {
    
    let mut addrs = std::mem::zeroed();
    syscall!(getifaddrs(&mut addrs));
    let mut cur = addrs;
    while !cur.is_null() {
        let cur_ = *cur;
        libc::printf("%s\n\0".as_ptr().cast(), cur_.ifa_name);
        cur = cur_.ifa_next;
    }
    libc::freeifaddrs(addrs);
}

#[test]
fn feature() {
    unsafe { get_default_route_ip() ; }
}
