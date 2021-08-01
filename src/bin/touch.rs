fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("touch: missing operand");
        unsafe { libc::exit(libc::EXIT_FAILURE) };
    }
    let filename = std::ffi::CString::new(args[1].as_str()).unwrap();
    let fd = unsafe {
        libc::open(
            filename.as_ptr(),
            libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH,
        )
    };
    if fd == -1 {
        unsafe {
            libc::perror(filename.as_ptr().cast());
            libc::exit(libc::EXIT_FAILURE);
        }
    }
    unsafe { libc::close(fd) };
}
