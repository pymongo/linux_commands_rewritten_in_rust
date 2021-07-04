#![warn(clippy::nursery, clippy::pedantic)]

use linux_commands_rewritten_in_rust::errno::last_errno_message;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("usage: chmod PERMISSION_BITS FILE");
        eprintln!("usage_example: chmod 777 main.rs");
        return;
    }
    if args[1].len() != 3 {
        eprintln!("Invalid PERMISSION_BITS={}", args[1]);
        eprintln!("usage_example: chmod 777 main.rs");
        return;
    }


    let permission_bits = args[1].clone().into_bytes();

    let user_permission = u32::from(permission_bits[0] - b'0');
    assert!(user_permission<=7);
    let user_permission = user_permission << 6;

    let group_permission = u32::from(permission_bits[1] - b'0');
    assert!(group_permission<=7);
    let group_permission = group_permission << 3;

    let other_permission = u32::from(permission_bits[2] - b'0');
    assert!(other_permission<=7);

    let mut mode = 0;
    for bit_mask in [libc::S_IRUSR, libc::S_IWUSR, libc::S_IXUSR] {
        if (user_permission & bit_mask) == bit_mask {
            mode |= bit_mask;
        }
    }
    for bit_mask in [libc::S_IRGRP, libc::S_IWGRP, libc::S_IXGRP] {
        if (group_permission & bit_mask) == bit_mask {
            mode |= bit_mask;
        }
    }
    for bit_mask in [libc::S_IROTH, libc::S_IWOTH, libc::S_IXOTH] {
        if (other_permission & bit_mask) == bit_mask {
            mode |= bit_mask;
        }
    }
    eprintln!("{:016b}", mode);


    let filename = format!("{}\0", args[2]);
    let ret = unsafe {
        libc::chmod(filename.as_ptr().cast(), mode)
    };
    if ret == -1 {
        eprintln!("{}", last_errno_message());
    }
}
