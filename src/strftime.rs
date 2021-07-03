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
output example: 2021-07-03 09:54:39

# Panics
if FFI call `localtime_r` or `strftime` failed, would panic
*/
#[allow(dead_code)]
#[must_use]
fn format_timestamp_to_ymd_hms(timestamp: libc::time_t) -> String {
    const BUFFER_LEN: usize = "2021-07-03 09:54:39\0".len();
    let mut buffer = [0_u8; BUFFER_LEN];
    let mut tm_struct = unsafe { std::mem::zeroed::<libc::tm>() };
    unsafe {
        if libc::localtime_r(&timestamp, (&mut tm_struct) as *mut _).is_null() {
            panic!("localtime_r failed: {}", std::io::Error::last_os_error());
        }
    };
    let len = unsafe {
        strftime(
            buffer.as_mut_ptr().cast(),
            BUFFER_LEN,
            "%Y-%m-%d %H:%M:%S\0".as_ptr().cast(),
            &tm_struct as *const _,
        )
    };
    assert_eq!(len, BUFFER_LEN - 1);
    unsafe { String::from_utf8_unchecked(buffer[..BUFFER_LEN - 1].to_vec()) }
}

#[test]
fn test_format_timestamp_to_ymd_hms() {
    const TEST_CASES: [(libc::time_t, &str); 1] = [(1625277279, "2021-07-03 09:54:39")];
    for (input, output) in TEST_CASES {
        assert_eq!(format_timestamp_to_ymd_hms(input), output);
    }
}

/**
output example: 2021-07-03 09:54:39.444706875 +0800

## Panics
if FFI call `localtime_r` or `strftime` failed, would panic

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
    const BUFFER_LEN: usize = "2021-07-03 09:54:39\0".len();
    let mut buffer = [0_u8; BUFFER_LEN];
    let mut tm_struct = unsafe { std::mem::zeroed::<libc::tm>() };
    unsafe {
        if libc::localtime_r(&timestamp, (&mut tm_struct) as *mut _).is_null() {
            panic!("localtime_r failed: {}", std::io::Error::last_os_error());
        }
    };
    let len = unsafe {
        strftime(
            buffer.as_mut_ptr().cast(),
            BUFFER_LEN,
            "%Y-%m-%d %H:%M:%S\0".as_ptr().cast(),
            &tm_struct as *const _,
        )
    };
    assert_eq!(len, BUFFER_LEN - 1);
    let ymd_hms = unsafe { String::from_utf8_unchecked(buffer[..BUFFER_LEN - 1].to_vec()) };
    let timezone = tm_struct.tm_gmtoff / 3600 + if tm_struct.tm_isdst > 0 { 1 } else { 0 };
    format!("{}.{} {:+03}00", ymd_hms, nanosecond, timezone)
}

#[test]
fn test_format_timestamp_with_nanosecond() {
    const TEST_CASES: [(libc::time_t, libc::time_t, &str); 1] =
        [(1625277279, 444706875, "2021-07-03 09:54:39.444706875 +0800")];
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
