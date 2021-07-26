use linux_commands_rewritten_in_rust::syscall;

fn main() {
    let mut uname = unsafe { std::mem::zeroed() };
    syscall!(uname(&mut uname));
    syscall!(printf("sysname=%s\n\0".as_ptr().cast(), uname.sysname));
    syscall!(printf(
        "nodename(hostname)=%s\n\0".as_ptr().cast(),
        uname.nodename
    ));
    syscall!(printf("release=%s\n\0".as_ptr().cast(), uname.release));
    // The version contains the date that kernel is compile
    syscall!(printf("version=%s\n\0".as_ptr().cast(), uname.version));
    syscall!(printf("machine=%s\n\0".as_ptr().cast(), uname.machine));
}
