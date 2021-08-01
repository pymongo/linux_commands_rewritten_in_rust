fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("rmdir: missing operand");
        unsafe { libc::exit(libc::EXIT_FAILURE) };
    }
    let filename = std::ffi::CString::new(args[1].as_str()).unwrap();
    if unsafe { libc::rmdir(filename.as_ptr()) } == -1 {
        unsafe {
            libc::perror(filename.as_ptr().cast());
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}
