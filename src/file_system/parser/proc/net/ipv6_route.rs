/// An entry in the ipv4 route table
#[derive(Debug, Clone)]
pub struct RouteEntry {
    /// Interface to which packets for this route will be sent
    pub iface: String,
    /// The destination network or destination host
    pub destination: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub flags: u16,
    /// Number of references to this route
    pub refcnt: u16,
    /// Count of lookups for the route
    pub r#use: u16,
    /// The 'distance' to the target (usually counted in hops)
    pub metrics: u32,
    pub mask: Ipv4Addr,
    /// Default maximum transmission unit for TCP connections over this route
    pub mtu: u32,
    /// Default window size for TCP connections over this route
    pub window: u32,
    /// Initial RTT (Round Trip Time)
    pub irtt: u32,
}

/// Reads the ipv4 route table
/// 
/// This data is from the `/proc/net/route` file
pub fn route() -> ProcResult<Vec<RouteEntry>> {
    let file = FileWrapper::open("/proc/net/route")?;
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    // First line is a header we need to skip
    for line in reader.lines().skip(1) {
        // Check if there might have been an IO error.
        let line = line?;
        let mut line = line.split_whitespace();
        // network interface name, e.g. eth0
        let iface = expect!(line.next());
        let destination = expect!(Ipv4Addr::from_str(expect!(line.next())));
        let gateway = expect!(Ipv4Addr::from_str(expect!(line.next())));
        let flags = from_str!(u16, expect!(line.next()), 16);
        let refcnt = from_str!(u16, expect!(line.next()), 10);
        let r#use = from_str!(u16, expect!(line.next()), 10);
        let metrics = from_str!(u32, expect!(line.next()), 10);
        let mask = expect!(Ipv4Addr::from_str(expect!(line.next())));
        let mtu = from_str!(u32, expect!(line.next()), 10);
        let window = from_str!(u32, expect!(line.next()), 10);
        let irtt = from_str!(u32, expect!(line.next()), 10);
        vec.push(RouteEntry {
            iface: iface.to_string(),
            destination,
            gateway,
            flags,
            refcnt,
            r#use,
            metrics,
            mask,
            mtu,
            window,
            irtt,
        });
    }

    Ok(vec)
}
