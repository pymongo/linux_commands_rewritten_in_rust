//! hostname (GNU inetutils)
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        _main();
    }
}

unsafe fn _main() {
    let mut buf = [0_u8; 128];
    libc::gethostname(buf.as_mut_ptr().cast(), 128);
    let hostname = String::from_utf8_unchecked(buf[..libc::strlen(buf.as_ptr().cast())].to_vec());
    println!("{}", hostname);
}
