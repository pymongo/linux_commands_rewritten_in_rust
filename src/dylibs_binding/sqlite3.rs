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
