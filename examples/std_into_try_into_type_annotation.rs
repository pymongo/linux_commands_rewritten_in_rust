/// 心疼 into/try_into 怎么就不能像 transmute 那样有 into/output 两个泛型参数
/// 每当有人说 turbofish 操作符难看时，我都会请他看看这个测试用例
/// https://github.com/rust-lang/rust/blob/48532096e0f03dc09f8bd15d5b2c98dfbd7e377a/src/test/ui/bastion-of-the-turbofish.rs
fn main() {
    let val = 1_i64;

    // match 语句内怎么给 try_into 标记类型?
    #[cfg(FALSE)]
    match val.try_into() {
        Ok(val) => {}
    }

    // 写法一
    let _ = u32::try_from(val);
    // 写法二
    let _ = TryInto::<u32>::try_into(val);
    // 写法三 使用 std::convert::identity API 辅助类型标注
    let a = std::convert::identity::<u32>(val.try_into().unwrap());
    dbg!(a);
}