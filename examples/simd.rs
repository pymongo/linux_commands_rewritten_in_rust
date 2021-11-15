#![feature(portable_simd)]
fn main() {
    let vector = std::simd::u64x8::from_array([1,2,3,4,5,6,7,8]);
    dbg!(vector * 10);
}