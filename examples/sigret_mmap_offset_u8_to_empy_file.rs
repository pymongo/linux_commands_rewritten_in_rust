/*!
## Runtime Error

## SIGENT 的可能原因:
- mmap空文件后，读取偏移为10的数据
- unimplemented instructions
*/
fn main() {
    const LEN: usize = 10;
    let fd = unsafe {
        libc::open(
            "/tmp/my_mmap_data\0".as_ptr().cast(),
            libc::O_RDWR | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    if fd == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
    // How to Fix: libc::write(fd, [0_u8; 10].as_ptr().cast(), 10);
    let mapped_addr = unsafe {
        libc::mmap(
            std::ptr::null_mut::<libc::c_void>(),
            LEN,
            libc::PROT_READ | libc::PROT_WRITE,
            // The segment changes are made in the file
            libc::MAP_SHARED,
            fd,
            0,
        )
    };
    if mapped_addr == libc::MAP_FAILED {
        panic!("{}", std::io::Error::last_os_error());
    }
    unsafe {
        libc::close(fd);
    }
    // Bug is here: read offset 10 to a empty file
    let _data = unsafe { *mapped_addr.cast::<[u8; LEN]>() };
    let ret = unsafe { libc::munmap(mapped_addr, LEN) };
    if ret == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
}
