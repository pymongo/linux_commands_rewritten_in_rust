#[link(name = "readline")]
extern "C" {
    pub static rl_readline_version: libc::c_int;
}
