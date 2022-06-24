/*!
## 「重要」`&T`和`*const T`，`&mut T`和`*mut T`是同一个类型

观察MIR代码可知，以下两种写法展开成的MIR源码是一样的:
> localtime_r(&timestamp, &mut tm_struct);
>
> localtime_r(&timestamp as *const i64, &mut tm_struct as *mut _);

参考 chrono 源码: <https://github.com/chronotope/chrono/blob/3467172c31188006147585f6ed3727629d642fed/src/sys/unix.rs#L84>

```text
if libc::localtime_r(&sec, &mut out).is_null() {
    panic!("localtime_r failed: {}", io::Error::last_os_error());
}
```
*/

// [Why libc remove strftime?](https://github.com/rust-lang/libc/commit/377ee7307ae7d4b6a5fb46802958518f724ba873)
extern "C" {
    fn strftime(
        buf: *mut libc::c_char,
        buf_len: libc::size_t,
        time_format: *const libc::c_char,
        tm_struct: *const libc::tm,
    ) -> libc::size_t;
    /// strptime not found in windows
    /// return the last one char  consumed in the conversion
    #[cfg(test)]
    fn strptime(
        s: *const libc::c_char,
        format: *const libc::c_char,
        tm_struct: *mut libc::tm,
    ) -> *const libc::c_char;
}

/// https://stackoverflow.com/questions/522251/whats-the-difference-between-iso-8601-and-rfc-3339-date-formats
/// chrono/time 的 RFC3339 的 +0000 要写成 +00:00
const RFC_3339_EXAMPLE: &str = "1970-01-01T00:00:00+0000\0";
const RFC_3339_LEN: usize = RFC_3339_EXAMPLE.len();
const RFC_3339_FORMAT: *const libc::c_char = "%Y-%m-%dT%H:%M:%S%z\0".as_ptr().cast();

/// s must end with nul
#[cfg(test)]
unsafe fn parse_rfc_3339_to_tm(s: &str) -> libc::tm {
    let mut tm = std::mem::zeroed();
    let parse_len = strptime(s.as_ptr().cast(), RFC_3339_FORMAT, &mut tm);
    assert_eq!(
        parse_len.offset_from(s.as_ptr().cast()),
        (s.len() - 1) as isize
    );
    tm
}

#[must_use]
pub fn tm_to_rfc_3339(tm: &libc::tm) -> String {
    let mut buffer = [0_u8; RFC_3339_LEN];
    let str_len = unsafe {
        strftime(
            buffer.as_mut_ptr().cast(),
            RFC_3339_LEN,
            RFC_3339_FORMAT,
            tm,
        )
    };
    assert_eq!(str_len, RFC_3339_LEN - 1);
    unsafe { String::from_utf8_unchecked(buffer[..RFC_3339_LEN - 1].to_vec()) }
}

#[test]
fn test_parse_rfc_3339() {
    unsafe {
        let now_i64 = libc::time(std::ptr::null_mut());
        let mut now_tm = std::mem::zeroed();
        libc::gmtime_r(&now_i64, &mut now_tm);
        let s = format!("{}\0", tm_to_rfc_3339(&now_tm));
        dbg!(&s);
        let mut tm2 = parse_rfc_3339_to_tm(&s);
        assert_eq!(now_tm.tm_sec, tm2.tm_sec);
        // mktime assumes that the date value is in the local time zone
        let timestamp = libc::mktime(&mut tm2);
        libc::localtime_r(&now_i64, &mut now_tm);
        assert_eq!(now_i64, timestamp + now_tm.tm_gmtoff);
    }
}

/**
output example: 2021-07-03 09:54:39.444706875 +0800

## daynight saving time example
when calc the timezone we need to know daynight saving time flag(`tm.is_dst`)

a example on mountain timezone
```text
timezone    getup_time
UTC         17:00
MST(UTC-07) 10:00 (without dst, 11/01-03/14)
MDT(UTC-06) 11:00 (summer with dst, 03/14-11/01)
```
*/
#[must_use]
pub fn format_timestamp_with_nanosecond(
    timestamp: libc::time_t,
    nanosecond: libc::time_t,
) -> String {
    let mut tm = unsafe { std::mem::zeroed() };
    unsafe {
        libc::localtime_r(&timestamp, &mut tm);
    }
    let mut ymd_hms = tm_to_rfc_3339(&tm).into_bytes();
    ymd_hms.truncate(ymd_hms.len() - "+0800".len());
    let ymd_hms = unsafe { String::from_utf8_unchecked(ymd_hms) };
    let timezone = tm.tm_gmtoff / 3600 + if tm.tm_isdst > 0 { 1 } else { 0 };
    format!("{}.{} {:+03}00", ymd_hms, nanosecond, timezone)
}

#[test]
fn test_format_timestamp_with_nanosecond() {
    const TEST_CASES: [(libc::time_t, libc::time_t, &str); 1] = [(
        1_625_277_279,
        444_706_875,
        "2021-07-03T09:54:39.444706875 +0800",
    )];
    for (timestamp, nanosecond, output) in TEST_CASES {
        assert_eq!(
            format_timestamp_with_nanosecond(timestamp, nanosecond),
            output
        );
    }
}

/*
## relative system call
- asctime(struct tm*): return example: `Sun Jun  9 12:34:56 2007\n\0`
- ctime: same as `asctime(localtime(timeval))`
*/
#[test]
fn test_asctime_and_ctime() {
    #[link(name = "c")]
    extern "C" {
        /// asctime include `\n`
        fn asctime(tm: *const libc::tm) -> *const libc::c_char;
        /// same as `asctime(localtime(time_t))`
        fn ctime(timestamp: *const libc::time_t) -> *const libc::c_char;
    }
    unsafe {
        libc::printf(
            "%s\0".as_ptr().cast(),
            asctime(libc::localtime(&libc::time(std::ptr::null_mut()))),
        );
        libc::printf(
            "%s\0".as_ptr().cast(),
            ctime(&libc::time(std::ptr::null_mut())),
        );
    }
}
