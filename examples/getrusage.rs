fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    // cpu usage = ((curr_usage.ru_utime+curr_usage.ru_stime) - (last_usage.ru_utime+last_usage.ru_stime)) / sample_interval
    // let sample_interval_ms = 5000;
    let ptr = libc::malloc(1 * 1024 * 1024 * 1024);
    assert!(!ptr.is_null());
    dbg!(std::process::id());

    let mut usage = std::mem::zeroed();
    libc::getrusage(libc::RUSAGE_SELF, &mut usage);
    dbg!(usage.ru_maxrss);
    libc::sleep(999);
}