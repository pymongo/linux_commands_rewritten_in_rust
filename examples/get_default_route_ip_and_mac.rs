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

unsafe fn get_default_network_interface_ip() -> std::net::Ipv4Addr {
    let mut machine_ip = std::mem::zeroed();
    // borrowed value not live long
    let default_route = proc::net::route::default_route_network_interface();

    let mut addrs = std::mem::zeroed();
    // return first address of Vec<ifaddrs>
    syscall!(getifaddrs(&mut addrs));
    let mut cur = addrs;
    while !cur.is_null() {
        let cur_deref = *cur;
        if cur_deref.ifa_addr.is_null() {
            cur = cur_deref.ifa_next;
            continue;
        }
        // sa_family is oneof AF_PACKET, AF_INET, AF_INET6
        if (*cur_deref.ifa_addr).sa_family != libc::AF_INET as u16 {
            cur = cur_deref.ifa_next;
            continue;
        }

        if libc::strcmp(cur_deref.ifa_name, default_route.as_ptr().cast()) == 0 {
            let addr = *cur_deref.ifa_addr.cast::<libc::sockaddr_in>();
            machine_ip = std::net::Ipv4Addr::from(addr.sin_addr.s_addr.to_ne_bytes());
            break;
        }
        cur = cur_deref.ifa_next;
    }
    libc::freeifaddrs(addrs);
    machine_ip
}

#[test]
fn a2() {
    let default_route = proc::net::route::default_route_network_interface();

    unsafe {
        libc::printf(
            "%s\n\0".as_ptr().cast(),
            default_route.as_ptr().cast::<libc::c_char>(),
        );
        dbg!(libc::strcmp(
            "wlp4s0\0".as_ptr().cast(),
            "wlp4s0\0".as_ptr().cast()
        ));
    }
}
