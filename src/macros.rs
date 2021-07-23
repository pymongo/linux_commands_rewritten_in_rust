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
