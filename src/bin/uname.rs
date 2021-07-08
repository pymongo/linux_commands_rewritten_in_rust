//! uname (GNU coreutils)
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        _main();
    };
}

unsafe fn _main() {
    let mut uname = std::mem::zeroed();
    assert_eq!(libc::uname(&mut uname), 0);
    libc::printf("sysname=%s\n\0".as_ptr().cast(), uname.sysname);
    libc::printf("nodename(hostname)=%s\n\0".as_ptr().cast(), uname.nodename);
    libc::printf("release=%s\n\0".as_ptr().cast(), uname.release);
    libc::printf("version=%s\n\0".as_ptr().cast(), uname.version);
    libc::printf("machine=%s\n\0".as_ptr().cast(), uname.machine);
}
