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
        crate::syscall!(getaddrinfo(
            host.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            &mut ret
        ));
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
pub fn dns_resolve(hostname: &str) -> libc::in_addr {
    let hostname_cstring = std::ffi::CString::new(hostname).unwrap();
    let hostent = unsafe { gethostbyname(hostname_cstring.as_ptr().cast()) };
    if hostent.is_null() {
        panic!("Invalid hostname");
    }
    let h_name = unsafe { (*hostent).h_name };
    let h_name_len = unsafe { libc::strlen(h_name) };
    let hostname_alias = unsafe { String::from_raw_parts(h_name.cast(), h_name_len, h_name_len) };
    println!("{} is an alias for {}", hostname_alias, hostname);
    let mut h_addr_list = unsafe { *hostent }.h_addr_list;
    let mut ipv4_addr_list = vec![];
    while !h_addr_list.is_null() {
        // let _in_addr_ptr = unsafe { (*h_addr_list).cast::<u32>() };
        let ipv4_addr_ptr = unsafe { (*h_addr_list).cast::<[u8; 4]>() };
        if ipv4_addr_ptr.is_null() {
            break;
        }
        let ipv4_addr = unsafe { *ipv4_addr_ptr };
        ipv4_addr_list.push(ipv4_addr);
        println!(
            "{} has address {}.{}.{}.{}",
            hostname_alias, ipv4_addr[0], ipv4_addr[1], ipv4_addr[2], ipv4_addr[3]
        );
        h_addr_list = unsafe { h_addr_list.add(1) };
    }
    std::mem::forget(hostname_alias);
    libc::in_addr {
        s_addr: unsafe { htonl(u32::from_be_bytes(ipv4_addr_list[0])) },
    }
}

#[test]
fn test_dns_resolve() {
    let _ = dns_resolve("www.rust-lang.org");
}

/// icmphdr.type usually use ICMP_ECHO
pub const ICMP_ECHO: u8 = 8;

#[repr(C)]
pub struct icmphdr {
    pub type_: u8,
    pub code: u8,
    pub checksum: u16,
    pub un: un,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union un {
    echo: echo,
    gateway: u32,
    frag: frag,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct echo {
    pub id: u16,
    pub sequence: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct frag {
    __glibc_reserved: u16,
    mtu: u16,
}

/// rfc792
#[must_use]
pub fn icmq_checksum(bytes: &[u8]) -> u16 {
    let mut sum = 0_u32;
    // skip type(u8) and code(u8) filed, because checksum initial value is 0, doesn't need to skip checksum field
    bytes.chunks_exact(2).skip(1).for_each(|u16_bytes| {
        sum += u32::from(u16::from_be_bytes(
            std::convert::TryInto::try_into(u16_bytes).unwrap(),
        ));
    });

    // sum = sum的高16位 + sum的低16位
    // 如果溢出(sum的高16位不为0)则继续，我这里偷懒了
    sum = (sum >> 16) + (sum & 0xffff);

    !sum as u16
}
