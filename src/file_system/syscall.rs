use std::os::raw::c_char;

#[link(name = "c")]
extern "C" {
    /// Both dirname() and basename() may modify the contents of path
    pub fn basename(path: *mut c_char) -> *mut c_char;
    pub fn dirname(path: *mut c_char) -> *mut c_char;
}
