/*!
## bus error
access memory beyond the physically address

## SIGBUS 的可能原因:
- mmap空文件后，读取偏移为10的数据显然超出"物理内存范围"，因为mmap是基于磁盘文件的数据，如果磁盘文件的大小为0，那么映射成的物理内存长度只能是0

*/

#[derive(Clone, Copy)]
#[repr(C)]
struct Byte(u8);

fn main() {
    const LEN: usize = 10;
    const SIZE: usize = std::mem::size_of::<Byte>();
    let fd = unsafe {
        libc::open(
            "/tmp/my_sigbus_example\0".as_ptr().cast(),
            libc::O_RDWR | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    if fd == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
    let mmap_len = LEN * SIZE;
    // How to Fix: libc::write(fd, [0_u8; 10].as_ptr().cast(), 10);
    let mapped_addr = unsafe {
        libc::mmap(
            std::ptr::null_mut::<libc::c_void>(),
            mmap_len,
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
    let _data = unsafe { *mapped_addr.cast::<[Byte; LEN]>() };
    let ret = unsafe { libc::munmap(mapped_addr, mmap_len) };
    if ret == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
}
