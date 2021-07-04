/*!
## How to get calling process uid/gid?
libc::getuid() and libc::getgid()

## how to find_uid_by_username?
libc::getpwnam(), libc::getpwnam_r() need to input user's password
reference: <https://stackoverflow.com/questions/39157675/how-to-get-linux-user-id-by-user-name>
*/

use linux_commands_rewritten_in_rust::errno::{last_errno, last_errno_message};

/// current  output:
/// expected output: uid=1000(w) gid=1001(w) groups=1001(w),998(wheel),991(lp),3(sys),90(network),98(power),1000(autologin),966(sambashare)
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage_example: id root");
        return;
    }
    let username = &args[1];
    let username_with_nul = std::ffi::CString::new(username.as_bytes()).unwrap();

    // passwd.pw_passwd always 'x' (passwd is hidden)
    let passwd = unsafe { libc::getpwnam(username_with_nul.as_ptr()) };
    if passwd.is_null() {
        if last_errno() == 0 {
            eprintln!("username({}) not found", username);
        } else {
            eprintln!("{}", last_errno_message());
        }
        return;
    }
    let passwd = unsafe { *passwd };
    println!("uid={uid}({username}) gid={gid}({username})", uid=passwd.pw_uid, gid=passwd.pw_gid, username=username);
}
