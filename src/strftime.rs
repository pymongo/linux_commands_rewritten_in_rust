/// [Why libc remove strftime?](https://github.com/rust-lang/libc/commit/377ee7307ae7d4b6a5fb46802958518f724ba873)
extern "C" {
    fn strftime(
        buf: *mut libc::c_char,
        buf_len: libc::size_t,
        time_format: *const libc::c_char,
        tm_struct: *const libc::tm,
    ) -> libc::size_t;
}

/**
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

## panics
system call localtime_r failed
*/
fn syscall_localtime_r(timestamp: i64) -> libc::tm {
    let mut tm_struct = unsafe { std::mem::zeroed() };
    unsafe {
        if libc::localtime_r(&timestamp, &mut tm_struct).is_null() {
            panic!("localtime_r failed: {}", std::io::Error::last_os_error());
        }
    };
    tm_struct
}

/// ## panics
/// system call strftime failed
fn strftime_ymd_hms(tm_struct: &libc::tm) -> String {
    const BUFFER_LEN: usize = "2021-07-03 09:54:39\0".len();
    let mut buffer = [0_u8; BUFFER_LEN];
    let len = unsafe {
        strftime(
            buffer.as_mut_ptr().cast(),
            BUFFER_LEN,
            "%Y-%m-%d %H:%M:%S\0".as_ptr().cast(),
            tm_struct,
        )
    };
    assert_eq!(len, BUFFER_LEN - 1);
    unsafe { String::from_utf8_unchecked(buffer[..BUFFER_LEN - 1].to_vec()) }
}

/// output example: 2021-07-03 09:54:39
#[allow(dead_code)]
#[must_use]
fn format_timestamp_to_ymd_hms(timestamp: i64) -> String {
    let tm_struct = syscall_localtime_r(timestamp);
    strftime_ymd_hms(&tm_struct)
}

#[test]
fn test_format_timestamp_to_ymd_hms() {
    const TEST_CASES: [(libc::time_t, &str); 1] = [(1_625_277_279, "2021-07-03 09:54:39")];
    for (input, output) in TEST_CASES {
        assert_eq!(format_timestamp_to_ymd_hms(input), output);
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
    let tm_struct = syscall_localtime_r(timestamp);
    let ymd_hms = strftime_ymd_hms(&tm_struct);
    let timezone = tm_struct.tm_gmtoff / 3600 + if tm_struct.tm_isdst > 0 { 1 } else { 0 };
    format!("{}.{} {:+03}00", ymd_hms, nanosecond, timezone)
}

#[test]
fn test_format_timestamp_with_nanosecond() {
    const TEST_CASES: [(libc::time_t, libc::time_t, &str); 1] =
        [(1_625_277_279, 444_706_875, "2021-07-03 09:54:39.444706875 +0800")];
    for (timestamp, nanosecond, output) in TEST_CASES {
        assert_eq!(
            format_timestamp_with_nanosecond(timestamp, nanosecond),
            output
        );
    }
}

// 2010-03-30T05:43:25.1234567890Z
// fn format_timestamp_to_utc_iso_8601(_timestamp: libc::time_t) -> String {
//     todo!()
// }
