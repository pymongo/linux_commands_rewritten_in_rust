use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct ProcNetRoute {
    pub iface: String,
    pub destination: Ipv4Addr,
    /// TODO support ipv6?
    pub gateway: Ipv4Addr,
}

/// if machine not connected to internet/router, all gateway is 0.0.0.0
#[must_use]
fn parse_proc_net_route() -> Vec<ProcNetRoute> {
    let mut routes = vec![];
    for line in std::fs::read_to_string("/proc/net/route")
        .unwrap()
        .lines()
        // skip header row
        .skip(1)
    {
        let row = line.split('\t').collect::<Vec<_>>();
        routes.push(ProcNetRoute {
            iface: row[0].to_string(),
            // parse naive endian bytes like `0112A8C0` -> `192.168.18.1`
            destination: u32::from_str_radix(row[1], 16)
                .unwrap()
                .to_ne_bytes()
                .try_into()
                .unwrap(),
            gateway: u32::from_str_radix(row[2], 16)
                .unwrap()
                .to_ne_bytes()
                .try_into()
                .unwrap(),
        });
    }
    routes
}

#[must_use]
pub fn default_route_network_interface() -> String{
    parse_proc_net_route()
    .into_iter()
    .find(|network_interface| {
        network_interface.gateway != Ipv4Addr::UNSPECIFIED
            && network_interface.destination == Ipv4Addr::UNSPECIFIED
    })
    .unwrap()
    .iface
}

#[test]
fn test_parse_proc_net_route() {
    dbg!(parse_proc_net_route());
}

#[cfg(test)]
mod parse_with_error_handling {
    fn default_route_network_interface() -> Option<String> {
        use std::net::Ipv4Addr;
        for line in std::fs::read_to_string("/proc/net/route")
            .ok()?
            .lines()
            .skip(1)
        {
            // let row = line.split('\t').collect::<Vec<_>>();
            let row = line.split_whitespace().collect::<Vec<_>>();
            let gateway: Ipv4Addr = u32::from_str_radix(row[2], 16)
                .ok()?
                .to_ne_bytes()
                .try_into()
                .ok()?;
            if gateway != Ipv4Addr::UNSPECIFIED {
                return Some(row[0].to_string());
            }
        }
        None
    }

    fn mac_address() -> Option<String> {
        Some(
            std::fs::read_to_string(format!(
                "/sys/class/net/{}/address",
                default_route_network_interface()?
            ))
            .ok()?
            .trim_end()
            .to_string(),
        )
    }

    #[test]
    fn test_default_route_network_interface() {
        dbg!(default_route_network_interface());
        dbg!(mac_address());
    }
}
