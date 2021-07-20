#![warn(clippy::nursery, clippy::pedantic)]
#[link(name = "c")]
extern "C" {
    /// extern char **environ;
    /// In C, `extern` mean `public static`, `static` mean `private static`
    static environ: *const *const libc::c_char;
}

unsafe fn traverse_env_vars() {
    // for (key, value) in std::env::vars()
    let mut env_vars = environ;
    while !(*env_vars).is_null() {
        libc::printf("%s\n\0".as_ptr().cast(), *env_vars);
        env_vars = env_vars.offset(1);
    }
}

fn main() {
    println!("version of `libreadline.so` = {}", unsafe {
        linux_commands_rewritten_in_rust::dylibs_binding::readline::rl_readline_version
    });
    unsafe {
        traverse_env_vars();
    }
}
