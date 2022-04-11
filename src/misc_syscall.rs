#[test]
fn test_gnu_get_libc_version() {
    extern "C" {
        fn gnu_get_libc_version() -> *const libc::c_char;
    }
    let version_cstr = unsafe { std::ffi::CStr::from_ptr(gnu_get_libc_version()) };
    dbg!(version_cstr.to_str().unwrap());
}
