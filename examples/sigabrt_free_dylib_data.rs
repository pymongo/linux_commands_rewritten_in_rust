use linux_programming::dylibs_binding::sqlite3::sqlite3_libversion;

fn main() {
    unsafe {
        let ptr = sqlite3_libversion() as *mut i8;
        let len = libc::strlen(ptr);
        let version = String::from_raw_parts(ptr.cast(), len, len);
        println!("found sqlite3 version={}", version);
        // Bug is here: String::drop try to free sqlite3 dylib string data cause signal SIGABRT
        // How to fix: mem::forget(version) or slice/ptr/strdup copy sqlite dylib data to Rust process
    }
}
