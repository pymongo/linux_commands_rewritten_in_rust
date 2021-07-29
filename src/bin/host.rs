use linux_commands_rewritten_in_rust::network::dns_resolve;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("host: missing hostname input");
        unsafe { libc::exit(libc::EXIT_FAILURE) };
    }
    let _ = dns_resolve(args[1].as_str());
}
