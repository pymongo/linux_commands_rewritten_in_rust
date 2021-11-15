use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct ProcNetRoute {
    pub iface: String,
    pub destination: Ipv4Addr,
    /// TODO support ipv6?
    pub gateway: Ipv4Addr,
}

#[must_use]
pub fn parse_proc_net_route() -> Vec<ProcNetRoute> {
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

#[test]
fn test_parse_proc_net_route() {
    dbg!(parse_proc_net_route());
}
