#[link(name = "crypto", kind = "dylib")]
extern "C" {
    /// unsigned char *MD5(const unsigned char *d, unsigned long n, unsigned char *md);
    fn MD5(input: *const u8, input_len: usize, output: &mut [u8; 16]) -> *mut u8;
}

#[must_use]
pub fn openssl_md5(input: &[u8]) -> String {
    let mut output = [0_u8; 16];
    unsafe {
        MD5(input.as_ptr().cast(), input.len(), &mut output);
    }
    let output = u128::from_be_bytes(output); // transmute 用的是 native_endian，最好还是显式的调用 from_be_bytes
    format!("{:x}", output)
}

#[test]
fn test_openssl_md5() {
    const MD5_TEST_CASES: [(&[u8], &str); 3] = [
        (
            b"The quick brown fox jumps over the lazy dog",
            "9e107d9d372bb6826bd81d3542a419d6",
        ),
        (
            b"The quick brown fox jumps over the lazy dog.",
            "e4d909c290d0fb1ca068ffaddf22cbd0",
        ),
        (b"", "d41d8cd98f00b204e9800998ecf8427e"),
    ];
    for (input, output) in MD5_TEST_CASES {
        assert_eq!(openssl_md5(input), output);
    }
}
