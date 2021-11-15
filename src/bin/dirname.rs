use linux_programming::file_system::syscall::dirname;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("dirname: missing operand");
        unsafe { libc::exit(libc::EXIT_FAILURE) };
    }
    let filename = std::ffi::CString::new(args[1].as_str()).unwrap();
    unsafe {
        libc::printf(
            "%s\n\0".as_ptr().cast(),
            dirname(filename.as_ptr() as *mut libc::c_char),
        );
    }
}
