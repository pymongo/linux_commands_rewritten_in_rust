//! check sqlite3 whether install: ldconfig -p | grep libsqlite3
#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_libversion() -> *const libc::c_char;
}

#[test]
fn test_sqlite3_libversion() {
    // copy version_str from libsqlite3.so
    let sqlite3_version = unsafe {
        std::ffi::CStr::from_ptr(sqlite3_libversion())
            .to_str()
            .unwrap()
    };
    dbg!(sqlite3_version);
}
