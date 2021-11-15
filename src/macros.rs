// 错误处理的脑洞:
// 为 -1 和 Null 实现 trait SyscallErr
// 然后用 rustc_hir 检查确保每一个 SyscallErr 的使用都用在 unsafe fn 或者 libc:: 上
#[macro_export]
macro_rules! syscall {
    ($fun:ident ( $($arg:expr),* $(,)* )) => {
        {
            #[allow(unused_unsafe)]
            let res = unsafe { libc::$fun($($arg),*) };
            if res == -1 {
                // Err(std::io::Error::last_os_error())
                panic!("{}", std::io::Error::last_os_error())
            } else {
                // Ok(res)
                res
            }
        }
    };
}

#[macro_export]
macro_rules! syscall_expr {
    ($expr:expr) => {{
        let res = unsafe { $expr };
        if res == -1 {
            panic!("{}", std::io::Error::last_os_error())
        } else {
            res
        }
    }};
}
