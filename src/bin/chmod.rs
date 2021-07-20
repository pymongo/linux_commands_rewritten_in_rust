#![warn(clippy::nursery, clippy::pedantic)]

use linux_commands_rewritten_in_rust::errno::last_errno_message;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("usage_example: chmod 777 main.rs");
        return;
    }

    let permission_bits = args[1].as_bytes();
    let user_permission = u32::from(permission_bits[0] - b'0');
    assert!(user_permission <= 7);
    let group_permission = u32::from(permission_bits[1] - b'0');
    assert!(group_permission <= 7);
    let other_permission = u32::from(permission_bits[2] - b'0');
    assert!(other_permission <= 7);

    let permission = (user_permission << 6) | (group_permission << 3) | other_permission;
    let filename = std::ffi::CString::new(args[2].as_bytes()).unwrap();
    let ret = unsafe { libc::chmod(filename.as_ptr(), permission) };
    if ret == -1 {
        eprintln!("{}", last_errno_message());
    }
}
