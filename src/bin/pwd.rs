//! pwd: shell built-in command

fn main() {
    const BUF_LEN: usize = 256;
    let mut buf = [0_u8; BUF_LEN];

    // getcwd is same as std::env::current_dir() or std::env::var("PWD")(in some shell)
    unsafe { libc::getcwd(buf.as_mut_ptr().cast(), BUF_LEN) };
    let pwd_str_len = unsafe { libc::strlen(buf.as_ptr().cast()) };
    let pwd_str = unsafe { String::from_utf8_unchecked(buf[..pwd_str_len].to_vec()) };

    // pwd_str is same as unsafe { std::ffi::CStr::from_ptr(buf.as_ptr().cast()) }
    println!("{}", pwd_str);
}
