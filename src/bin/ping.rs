use linux_programming::network::{dns_resolve, icmphdr, icmq_checksum, ICMP_ECHO};
use linux_programming::{syscall, SOCKADDR_IN_LEN};

const PACKET_LEN: usize = 64;

#[repr(C)]
struct Packet {
    hdr: icmphdr,
    msg: [u8; PACKET_LEN - std::mem::size_of::<icmphdr>()],
}

#[allow(clippy::cast_possible_truncation)]
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("ping: missing operand");
        unsafe { libc::exit(libc::EXIT_FAILURE) };
    }

    let hostname = args[1].as_str();
    println!("{}", hostname);
    let addr = dns_resolve(hostname);
    let remote_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: 0,
        sin_addr: addr,
        sin_zero: unsafe { std::mem::zeroed() },
    };

    /*
    use **sysctl** check ping permission
    ```
    [w@ww ~]$ sysctl net.ipv4.ping_group_range
    net.ipv4.ping_group_range = 0   2147483647
    ```
    ubuntu/android allow any group_id(0-2147483647) to send ping in UDP protocol
    ping 命令用的 ICMP 一般是 SOCK_RAW 或 UDP
    如果 linux 系统 sysctl 配置的 net.ipv4.ping_group_range 不包含当前用户的所在组
    那只能通过 root 权限的 SOCK_RAW 发 ping 数据包，例如 `sudo -E cargo run`
    */
    let socket_fd = syscall!(socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_ICMP));
    syscall!(fcntl(socket_fd, libc::F_SETFL, libc::O_NONBLOCK));
    syscall!(setsockopt(
        socket_fd,
        libc::SOL_IP,
        libc::IP_TTL,
        (&64 as *const i32).cast(),
        std::mem::size_of::<i32>() as u32
    ));

    for _ in 0..10 {
        let mut packet: Packet = unsafe { std::mem::zeroed() };
        let mut addr = remote_addr;
        let mut addrlen = SOCKADDR_IN_LEN;
        let recvfrom_ret = unsafe {
            libc::recvfrom(
                socket_fd,
                (&mut packet as *mut Packet).cast(),
                PACKET_LEN,
                0,
                (&mut addr as *mut libc::sockaddr_in).cast(),
                &mut addrlen,
            )
        };
        if recvfrom_ret > 0 {
            println!("ping success");
            std::process::exit(libc::EXIT_SUCCESS);
        }

        packet = unsafe { std::mem::zeroed() };
        packet.hdr.type_ = ICMP_ECHO;
        for i in 0..packet.msg.len() - 1 {
            packet.msg[i] = i as u8 + b'0';
        }
        packet.hdr.checksum = icmq_checksum(&packet.msg);

        syscall!(sendto(
            socket_fd,
            (&packet as *const Packet).cast(),
            PACKET_LEN,
            0,
            (&remote_addr as *const libc::sockaddr_in).cast::<libc::sockaddr>(),
            SOCKADDR_IN_LEN,
        ));

        syscall!(usleep(300 * 1000));
    }
    eprintln!("ping failed!");
}
