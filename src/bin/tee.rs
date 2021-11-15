use linux_programming::syscall;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("tee: missing operand");
        eprintln!("usage example: cat Cargo.toml | tee Cargo.toml.bak");
        unsafe { libc::exit(libc::EXIT_FAILURE) };
    }

    let mut pipes_1 = [-1; 2];
    let mut pipes_2 = [-1; 2];
    syscall!(pipe(pipes_1.as_mut_ptr()));
    syscall!(pipe(pipes_2.as_mut_ptr()));

    // STDIN -> pipes_1[write] -> pipes_1[read]
    syscall!(splice(
        libc::STDIN_FILENO,
        std::ptr::null_mut(),
        pipes_1[1],
        std::ptr::null_mut(),
        libc::PIPE_BUF,
        0
    ));
    // pipes_1[read].copy() -> pipes_2[write] -> pipes_2[read]
    syscall!(tee(pipes_1[0], pipes_2[1], libc::PIPE_BUF, 0));

    // pipes_1[read] -> STDOUT
    syscall!(splice(
        pipes_1[0],
        std::ptr::null_mut(),
        libc::STDOUT_FILENO,
        std::ptr::null_mut(),
        libc::PIPE_BUF,
        0
    ));

    let filename = std::ffi::CString::new(args[1].as_str()).unwrap();
    let fd = syscall!(open(
        filename.as_ptr(),
        libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
        0o644
    ));
    // pipes_2[read] -> fd
    syscall!(splice(
        pipes_2[0],
        std::ptr::null_mut(),
        fd,
        std::ptr::null_mut(),
        libc::PIPE_BUF,
        0
    ));
}
