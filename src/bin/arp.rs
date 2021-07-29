fn main() {
    let f = unsafe { libc::fopen("/proc/net/arp\0".as_ptr().cast(), "r\0".as_ptr().cast()) };
    let mut line_buf = [0_u8; libc::BUFSIZ as usize];
    loop {
        let line = unsafe { libc::fgets(line_buf.as_mut_ptr().cast(), line_buf.len() as i32, f) };
        if line.is_null() {
            break;
        }
        unsafe {
            libc::printf("%s\0".as_ptr().cast(), line);
        }
    }
}
