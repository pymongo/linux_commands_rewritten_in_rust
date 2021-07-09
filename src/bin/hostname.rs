//! hostname (GNU inetutils)
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut buf = [0_u8; 128];
    libc::gethostname(buf.as_mut_ptr().cast(), 128);
    let hostname = String::from_utf8_unchecked(buf[..libc::strlen(buf.as_ptr().cast())].to_vec());
    println!("{}", hostname);
}

#[test]
fn test_gethostid() {
    extern "C" {
        /// License managers use this to ensure that
        /// software programs can run only on machines that hold valid licenses
        fn gethostid() -> i64;
    }
    dbg!(unsafe { gethostid() });
}
