/**
run 100 tims calc mean run time: perf stat -r 100 ./target/debug/tree
*/
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
    unsafe {
        libc::chdir(input_filename_cstr);
    }
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

        // skip current directory and parent directory
        if libc::strcmp(filename_cstr, ".\0".as_ptr().cast()) == 0
            || libc::strcmp(filename_cstr, "..\0".as_ptr().cast()) == 0
        {
            continue;
        }

        // check file whether a directory
        let mut stat_buf = std::mem::zeroed();
        // lstat doesn't follow link
        linux_commands_rewritten_in_rust::syscall!(lstat(filename_cstr, &mut stat_buf));
        let is_dir = (stat_buf.st_mode & libc::S_IFMT) == libc::S_IFDIR;

        // convert filename from [c_char; NAME_MAX] to String
        let filename_string = String::from_raw_parts(
            (filename_cstr as *mut i8).cast(),
            libc::strlen(filename_cstr),
            linux_commands_rewritten_in_rust::NAME_MAX,
        );
        println!(
            "{}{}{}",
            " ".repeat(indent),
            filename_string,
            if is_dir { "/" } else { "" }
        );
        std::mem::forget(filename_string);

        if is_dir {
            // backtracking: opendir<->closedir, chdir(filename_cstr)<->chdir("..\0")
            let dirp_inner_dir = libc::opendir(filename_cstr);
            libc::chdir(filename_cstr);
            traverse_dir_dfs(dirp_inner_dir, indent + 4);
            libc::chdir("..\0".as_ptr().cast());
            libc::closedir(dirp_inner_dir);
            // set ptr to null after tree to prevent double free
            // dirp_inner_dir = std::ptr::null_mut();
        }
    }
}
