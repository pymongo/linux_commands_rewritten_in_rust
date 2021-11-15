use linux_commands_rewritten_in_rust::syscall;

/**
## multi ways to get hostname in Linux
- libc::gethostname()
- libc::uname()
- uname --nodename
- hostname
- cat /etc/hostname
- cat /proc/sys/kernel/hostname
*/
fn main() {
    let mut buf = [0_u8; 64];
    syscall!(gethostname(buf.as_mut_ptr().cast(), 128));
    syscall!(printf(
        "%s\n\0".as_ptr().cast(),
        buf.as_ptr().cast::<libc::c_char>()
    ));
}

#[cfg(FALSE)]
unsafe fn get_hostname_by_gethostname_syscall() -> String {
    let mut buf = [0_u8; 256];
    libc::gethostname(buf.as_mut_ptr().cast(), buf.len());
    let len = libc::strlen(buf.as_ptr().cast());
    String::from_utf8_unchecked(buf[..len].to_vec())
}
