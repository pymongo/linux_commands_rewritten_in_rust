//! ls (GNU coreutils)
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_filename = if let Some(filename) = args.get(1) {
        format!("{}\0", filename) // or std::ffi::CString::new
    } else {
        ".\0".to_string()
    };

    let dir = unsafe { libc::opendir(input_filename.as_ptr().cast()) };
    loop {
        let dir_entry = unsafe { libc::readdir(dir) };
        if dir_entry.is_null() {
            // directory_entries iterator end
            break;
        }
        let filename_str = unsafe {
            let dir_entry = *dir_entry;
            let filename_len = libc::strlen(dir_entry.d_name.as_ptr());
            //let filename_bytes = std::mem::transmute::<&[i8], &[u8]>(&dir_entry.d_name[..filename_len]);
            let filename_bytes =
                &*(&dir_entry.d_name[..filename_len] as *const [i8] as *const [u8]);
            String::from_utf8_unchecked(filename_bytes.to_owned())
        };
        println!("{}", filename_str);
    }
}
