/*!
`route` or `ip route` command alternative impl
TODO do A/B testing with ip route command output?
*/

fn main() {
    dbg!(std::fs::read_to_string("/proc/net/route").unwrap());
}
