#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_filename = if let Some(filename) = args.get(1) {
        format!("{}\0", filename) // or std::ffi::CString::new
    } else {
        ".\0".to_string()
    };

    let dirp = unsafe { libc::opendir(input_filename.as_ptr().cast()) };
    if dirp.is_null() {
        unsafe {
            libc::perror(input_filename.as_ptr().cast());
        }
        return;
    }
    loop {
        let dir_entry = unsafe { libc::readdir(dirp) };
        if dir_entry.is_null() {
            // directory_entries iterator end
            break;
        }
        unsafe {
            let dir_entry = *dir_entry;
            libc::printf("%s\n\0".as_ptr().cast(), dir_entry.d_name);
        }
    }
}
