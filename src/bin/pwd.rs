/**
getcwd aka get current working directory
getcwd is same as std::env::current_dir() or std::env::var("PWD")(in some shell)

## How to get first nul_byte(b'\0') in bytes if system_call doesn't tell you modified len
example system call: getcwd, strerror_r

One solution is `unsafe { std::ffi::CStr::from_ptr(buf.as_ptr().cast()) }`

other better alternatives:
1. let pwd_str_len = buf.iter().position(|&x| x == b'\0').unwrap();
2. let pwd_str_len = unsafe { libc::strlen(buf.as_ptr().cast()) };
*/
use linux_programming::{syscall, NAME_MAX};

fn main() {
    let mut buf = [0_u8; NAME_MAX];
    unsafe { libc::getcwd(buf.as_mut_ptr().cast(), buf.len()) };
    syscall!(printf(
        "%s\n\0".as_ptr().cast(),
        buf.as_ptr().cast::<libc::c_char>()
    ));
}
