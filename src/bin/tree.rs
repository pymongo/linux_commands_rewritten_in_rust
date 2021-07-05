//! tree <https://archlinux.org/packages/extra/x86_64/tree/>
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_filename = if let Some(filename) = args.get(1) {
        format!("{}\0", filename)
    } else {
        ".\0".to_string()
    };

    let input_filename_cstr = input_filename.as_ptr().cast();
    let dirp = unsafe { libc::opendir(input_filename_cstr) };
    if dirp.is_null() {
        unsafe {
            libc::perror(input_filename_cstr);
        }
        return;
    }
    unsafe { libc::chdir(input_filename_cstr) };
    unsafe {
        traverse_dir_dfs(dirp, 0);
    }
}

unsafe fn traverse_dir_dfs(dirp: *mut libc::DIR, indent: usize) {
    loop {
        let dir_entry = libc::readdir(dirp);
        if dir_entry.is_null() {
            return;
        }
        let dir_entry = *dir_entry;
        let filename_cstr = dir_entry.d_name.as_ptr();

        // 1. skip current directory and parent directory
        if libc::strcmp(filename_cstr, ".\0".as_ptr().cast()) == 0
            || libc::strcmp(filename_cstr, "..\0".as_ptr().cast()) == 0
        {
            continue;
        }

        // 2. check file whether a directory
        let mut stat_buf = std::mem::zeroed();
        // lstat doesn't follow link, prevent a link b and b link a cause infinite loop
        libc::lstat(filename_cstr, &mut stat_buf);
        let is_dir = stat_buf.st_mode & libc::S_IFMT == libc::S_IFDIR;

        // 3. convert filename from [c_char; 256] to String
        let filename_len = libc::strlen(filename_cstr);
        let filename_bytes = &*(&dir_entry.d_name[..filename_len] as *const [i8] as *const [u8]);
        let filename_string = String::from_utf8_unchecked(filename_bytes.to_owned());
        println!(
            "{}{}{}",
            " ".repeat(indent),
            filename_string,
            if is_dir { "/" } else { "" }
        );

        if is_dir {
            // backtracking: opendir<->closedir, chdir(..)<->chdir("..\0")
            let dirp2 = libc::opendir(filename_cstr);
            libc::chdir(filename_cstr);
            traverse_dir_dfs(dirp2, indent + 4);
            libc::chdir("..\0".as_ptr().cast());
            libc::closedir(dirp);
        }
    }
}
