/*!
## 说下我是怎么学习 errno 的方法论和过程
1. 发现mkfifo打开文件路径字符串末尾没带'\0'导致打开失败问题(一开始还不知道是缺少nul byte的原因)
2. 查看nix库mkfifo的示例及其源码
3. 发现nix库会认为系统调用返回-1就是系统调用失败
4. 再发现nix库遇到系统调用失败时会用libc::__errno_location()查找错误原因
5. __errno_location()会跳转到errno词条的man7.org文档，详细学习了errno原理
6. 标准库源码搜索errno_location(): last_os_error()调用了errno()，errno()调用那个location
7. 再看标准库impl fmt::Debug for Repr源码，发现libc::strerror_r能把errno数字变err_msg字符串
8. 后来发现很多linux底层库都用last_os_error()处理系统调用失败的情况

## errno文档解读
https://man7.org/linux/man-pages/man3/errno.3.html

> system calls and some library functions in the event of an error to indicate what went wrong
>
> -1 from most system calls; -1 or NULL from most library functions

大意是一些库或系统调用返回-1或NULL调试调用出错，系统调用通常返回-1表示调用失败，这时候可以找errno查看错误码确定错误原因

> error numbers using the errno(1) command(part of the moreutils package)

补充说明，出错时可以调用`__errno_location()`函数获取最近一次系统调用的错误码

可以用errno命令解读错误码数字的详细含义，也可以用strerror_r将errno转换为错误信息的字符串

> errno is thread-local

## errno should copy immediately

除了C标准库函数，还有很多库都会修改extern int errno，包括Rust的print(毕竟调用了libc::write)

## errno错误码示例

### ENOENT 2 No such file or directory
可能的错误原因:
- 路径不存在
- 路径字符串不合法: **C语言的字符串没加\0作为终止符**

### ENOMEM 12 Cannot allocate memory
注意标准库的Error没有解析错误码12，所以标准库没有像C语言那样能处理内存分配失败的情况(失败就panic，C一般通过malloc的返回值是否为null处理内存申请失败)

可能的错误原因:
- io_uring not enough lockable memory, please increase memlock config in /etc/security/limits.conf

### EADDRINUSE 98 Address already in use

### 其它错误
- disk is full
- too many open files(ulimits max open fd)
*/
#[must_use]
pub fn last_errno() -> i32 {
    // std::io::Error::last_os_error().raw_os_error().unwrap()
    unsafe { *libc::__errno_location() }
}

/// ## Panics
/// system call strerror_r failed
#[must_use]
#[inline]
pub fn last_errno_message() -> String {
    let errno = last_errno();
    errno_err_msg(errno).unwrap()
}

/**
## Panics
panic on strerror_r failed
## Errors
Invalid errno input
*/
pub fn errno_err_msg(errno: i32) -> Result<String, std::io::ErrorKind> {
    const BUF_LEN: usize = 128;
    let mut buf = [0_u8; BUF_LEN];
    let ret = unsafe { libc::strerror_r(errno, buf.as_mut_ptr().cast(), BUF_LEN) };
    if ret == libc::EINVAL {
        // EINVAL 22 Invalid argument
        return Err(std::io::ErrorKind::InvalidInput);
    }
    assert_eq!(ret, 0);

    let err_msg_buf_len = unsafe { libc::strlen(buf.as_ptr().cast()) };
    let err_msg = unsafe { String::from_utf8_unchecked(buf[..err_msg_buf_len].to_vec()) };
    Ok(err_msg)
}
