//! convert some system call return type to Rust type
//! example getcwd "return" [c_char: N], convert it to Rust friendly String

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
pub fn getcwd() -> String {
    const BUF_LEN: usize = 256;
    let mut buf = [0_u8; BUF_LEN];
    unsafe {
        libc::getcwd(buf.as_mut_ptr().cast(), BUF_LEN);
        String::from_utf8_unchecked(buf[..libc::strlen(buf.as_ptr().cast())].to_vec())
    }
}
