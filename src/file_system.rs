use std::os::raw::c_char;

#[link(name = "c")]
extern "C" {
    /// Both dirname() and basename() may modify the contents of path
    pub fn basename(path: *mut c_char) -> *mut c_char;
    pub fn dirname(path: *mut c_char) -> *mut c_char;
}

/// print example:
/// usage: host $hostname
pub fn print_executable_usage(argv_0: &str, usage: &str) {
    let exe_name = std::ffi::CString::new(argv_0).unwrap();
    let executable_name = unsafe { basename(exe_name.as_ptr() as *mut c_char) };
    let len = unsafe { libc::strlen(executable_name) };
    let executable_name = unsafe { String::from_raw_parts(executable_name.cast(), len, len) };
    eprintln!("usage: {} {}", executable_name, usage);
    std::mem::forget(executable_name);
}
