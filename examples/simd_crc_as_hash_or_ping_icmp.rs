use std::arch::x86_64::_mm_crc32_u16;

/// CRC 循环冗余校验码 算法
/// ping/icmp 的 checksum: u16 就是用 CRC 算出来的
fn main() {
    unsafe {
        dbg!(_mm_crc32_u16(0, 1));
    }
}
