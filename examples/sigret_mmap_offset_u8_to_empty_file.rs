/*!
## Runtime Error

## SIGENT 的可能原因:
- mmap空文件后，读取偏移为10的数据
- unimplemented instructions
*/
use linux_programming::syscall;
fn main() {
    const LEN: usize = 10;
    let fd = syscall!(open(
        "/tmp/my_mmap_data\0".as_ptr().cast(),
        libc::O_RDWR | libc::O_CREAT,
        libc::S_IRUSR | libc::S_IWUSR,
    ));
    // How to Fix: libc::write(fd, [0_u8; 10].as_ptr().cast(), 10); or set MAP_ANONYMOUS flag
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
    syscall!(munmap(mapped_addr, LEN));
}
