use libc::{c_int, c_short, c_void, timeval};

#[link(name = "event")]
extern "C" {
    pub type event;
    /// event_type is similar to pollfd.events
    pub fn event_new(
        base: *mut event_base,
        fd: c_int,
        event_type: c_short,
        cb: extern "C" fn(fd: c_int, event_type: c_short, cb_arg: *mut c_void),
        cb_arg: *mut c_void,
    ) -> *mut event;
    pub fn event_add(ev: *mut event, timeout: *const timeval) -> c_int;
    pub fn event_free(ev: *mut event);
    /// use in signal event arg
    pub fn event_self_cbarg() -> *mut c_void;
    /// Reactor
    pub type event_base;
    pub fn event_init() -> *mut event_base;
    pub fn event_base_loop(base: *mut event_base, flags: c_int) -> c_int;
    pub fn event_base_dispatch(base: *mut event_base) -> c_int;
    pub fn event_base_free(base: *mut event_base);
    pub fn event_base_loopexit(base: *mut event_base, delay: *const timeval) -> c_int;
}

/// event_type
pub const EV_WRITE: c_short = 0x40;
pub const EV_SIGNAL: c_short = 0x80;
/// run forever
pub const EV_PERSIST: c_short = 0x10;
pub const EV_ET: c_short = 0x20;

#[cfg(test)]
extern "C" fn sigint_cb(_fd: i32, _event_type: c_short, _cb_arg: *mut c_void) {
    println!("get SIGINT Ctrl+C");
    // let base = cb_arg as *mut event_base;
    // let delay = timeval {
    //     tv_sec: 1,
    //     tv_usec: 0,
    // };
    // crate::syscall_expr!(event_base_loopexit(base, &delay));
}

#[cfg(test)]
extern "C" fn timeout_cb(_fd: i32, _event_type: c_short, _cb_arg: *mut c_void) {
    println!("run interval in 1s...");
}

#[test]
fn test_libevent() {
    unsafe {
        let base = event_init();
        assert!(!base.is_null());

        // FIXME SIGINT callback not working
        let sigint_event = event_new(
            base,
            libc::SIGINT,
            EV_SIGNAL | EV_PERSIST,
            sigint_cb,
            event_self_cbarg(),
        );
        assert!(!sigint_event.is_null());
        assert_ne!(event_add(sigint_event, std::ptr::null()), -1);

        let timeout_event = event_new(base, -1, EV_PERSIST, timeout_cb, std::ptr::null_mut());
        assert!(!timeout_event.is_null());
        let interval = timeval {
            tv_sec: 1,
            tv_usec: 0,
        };
        assert_ne!(event_add(timeout_event, &interval), -1);

        assert_ne!(event_base_dispatch(base), -1);
        // or event_base_loop(base, 0);

        event_free(sigint_event);
        event_base_free(base);
    }
}
