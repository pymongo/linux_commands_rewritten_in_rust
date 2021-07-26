use libc::{c_char, c_int, in_addr};

/**
```text
MariaDB [test]> select inet_aton("192.168.1.1");
+--------------------------+
| inet_aton("192.168.1.1") |
+--------------------------+
|               3232235777 |
+--------------------------+
MariaDB [test]> select inet_ntoa(3232235777);
```
*/
#[link(name = "c")]
extern "C" {
    /// aton: means string to network_ip
    /// ntoa: means network_ip to string
    pub fn inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int;
    pub fn inet_ntoa(in_: in_addr) -> *mut c_char;
    // inet_addr 是 inet_aton 完全不考虑字符串解析错误的版本
    // fn inet_addr(cp: *const c_char) -> in_addr_t
    pub fn gethostbyname(name: *const c_char) -> *mut libc::hostent;
    /// htons: H(host byte order) TO N(network byte order) S(short)
    /// network byte order == MSB == bigger-endian
    pub fn htonl(hostlong: u32) -> u32;
    pub fn htons(hostshort: u16) -> u16;
    /// License managers use this to ensure that
    /// software programs can run only on machines that hold valid licenses
    pub fn gethostid() -> i64;
}

/// The getaddrinfo() function combines the functionality  provided  by  the  gethostbyname(3) and getservbyname(3) functions into a single interface
#[cfg(test)]
fn dns_lookup_getaddrinfo(hostname: &str) {
    let host = std::ffi::CString::new(hostname).unwrap();
    unsafe {
        // ret: Vec<addrinfo>
        let mut ret = std::ptr::null_mut();
        libc::getaddrinfo(host.as_ptr(), std::ptr::null(), std::ptr::null(), &mut ret);
        let sockaddr = *((*ret).ai_addr);
        let mut ipv4 = [0_u8; 4];
        #[allow(clippy::transmute_ptr_to_ptr)]
        ipv4.copy_from_slice(std::mem::transmute(&sockaddr.sa_data[2..6]));
        println!("{} = {:?}", hostname, ipv4);
        libc::freeaddrinfo(ret);
    }
}

#[test]
fn test_dns_lookup_getaddrinfo() {
    dns_lookup_getaddrinfo("localhost");
}

#[must_use]
pub fn in_addr_to_string(in_addr: libc::in_addr) -> String {
    unsafe {
        std::ffi::CString::from_raw(libc::strdup(inet_ntoa(in_addr)))
            .to_str()
            .unwrap()
            .to_string()
    }
}

#[must_use]
pub fn dns_lookup_gethostbyname(hostname: &str) -> Vec<libc::in_addr> {
    let host = std::ffi::CString::new(hostname).unwrap();
    let hostents = unsafe { gethostbyname(host.as_ptr().cast()) };
    if hostents.is_null() {
        panic!("Invalid hostname");
    }
    let mut in_addr_list = vec![];
    let mut addr_bytes_list = unsafe { *hostents }.h_addr_list;
    while !addr_bytes_list.is_null() {
        #[allow(clippy::cast_ptr_alignment)]
        let in_addr_ptr = unsafe { (*addr_bytes_list).cast::<libc::in_addr>() };
        if in_addr_ptr.is_null() {
            break;
        }
        let in_addr = unsafe { *in_addr_ptr };
        in_addr_list.push(in_addr);
        addr_bytes_list = unsafe { addr_bytes_list.add(1) };
    }
    in_addr_list
}

#[test]
fn test_dns_lookup_gethostbyname() {
    let addr_list = dns_lookup_gethostbyname("www.rust-lang.org");
    for addr in addr_list {
        dbg!(in_addr_to_string(addr));
    }
}
