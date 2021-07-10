/*!
Linux would terminal process when process dereference of a invalid address(SIGSEGV)

## segment fault
process try to access memory it doesn't own

## SIGSEGV 的可能原因:
- dereference NULL or invalid_address, eg. readdir(NULL)
- use-after-free(danling pointers): access de-allocated memory
- using uninitialized pointer
- access memory process doesn't own, eg. index out of range

*/
fn main() {
    let input_filename = format!("{}/Cargo.toml\0", env!("CARGO_MANIFEST_DIR"));
    // Bug is here: should check dirp.is_null(). if input_filename not a dir, dirp would be NULL
    let dirp = unsafe { libc::opendir(input_filename.as_ptr().cast()) };
    // How to fix: if dirp.is_null() { panic!() }
    loop {
        // `Segmentation fault (core dumped)` exit code 139 (interrupted by signal 11: SIGSEGV)
        let dir_entry = unsafe { libc::readdir(dirp) };
        if dir_entry.is_null() {
            break;
        }
        let filename_str = unsafe {
            let dir_entry = *dir_entry;
            let filename_len = libc::strlen(dir_entry.d_name.as_ptr());
            let filename_bytes =
                &*(&dir_entry.d_name[..filename_len] as *const [i8] as *const [u8]);
            String::from_utf8_unchecked(filename_bytes.to_owned())
        };
        println!("{}", filename_str);
    }
}
