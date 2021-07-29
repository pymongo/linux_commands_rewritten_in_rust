fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("cat: missing operand");
        unsafe { libc::exit(libc::EXIT_FAILURE) };
    }
    let filename = std::ffi::CString::new(args[1].as_str()).unwrap();
    let f = unsafe { libc::fopen(filename.as_ptr().cast(), "r\0".as_ptr().cast()) };
    if f.is_null() {
        unsafe {
            libc::perror(std::ptr::null());
            libc::exit(libc::EXIT_FAILURE);
        }
    }
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
