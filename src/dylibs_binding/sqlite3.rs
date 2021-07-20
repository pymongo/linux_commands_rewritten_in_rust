//! check sqlite3 whether install: ldconfig -p | grep libsqlite3
#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_libversion() -> *const libc::c_char;
}

#[test]
fn test_sqlite3_libversion() {
    unsafe {
        libc::printf(
            "sqlite3_libversion() = %s\n\0".as_ptr().cast(),
            sqlite3_libversion(),
        );
    }
}

#[test]
fn test_sqlite() {
    unsafe {
        let ptr = sqlite3_libversion() as *mut i8;
        let _sqlite_version = String::from_raw_parts(ptr.cast(), "3.23.0".len(), 10);
        // Bug is here: String::drop try to free sqlite3 dylib string data cause signal SIGABRT
        // How to fix: mem::forget(_sqlite_version) or ptr::copy/strdup copy sqlite dylib data to Rust process
    }
}
