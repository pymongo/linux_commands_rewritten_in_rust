/*!
Linux would terminal process when process dereference of a invalid address(SIGSEGV)

## segment fault
process try to access memory it doesn't own

## SIGSEGV 的可能原因:
- dereference NULL or invalid_address, eg. readdir(NULL)
- dereference to System V shared memory not attach or after detach
- stack overflow
- use-after-free(danling pointers): access de-allocated memory
- using uninitialized pointer
- access memory process doesn't own, eg. index out of range
*/
fn main() {
    let input_filename = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml\0");
    // Bug is here: should check dirp.is_null(). if input_filename not a dir, dirp would be NULL
    let dirp = unsafe { libc::opendir(input_filename.as_ptr().cast()) };
    // How to fix: if dirp.is_null() { panic!() }
    loop {
        // `Segmentation fault (core dumped)` exit code 139 (interrupted by signal 11: SIGSEGV)
        let dir_entry = unsafe { libc::readdir(dirp) };
        if dir_entry.is_null() {
            break;
        }
        unsafe {
            let dir_entry = *dir_entry;
            libc::printf("%s\n\0".as_ptr().cast(), dir_entry.d_name);
        }
    }
}
