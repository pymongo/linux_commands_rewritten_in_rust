use linux_commands_rewritten_in_rust::syscall;

fn main() {
    let mut buf = [0_u8; 64];
    syscall!(gethostname(buf.as_mut_ptr().cast(), 128));
    syscall!(printf(
        "%s\n\0".as_ptr().cast(),
        buf.as_ptr().cast::<libc::c_char>()
    ));
}
