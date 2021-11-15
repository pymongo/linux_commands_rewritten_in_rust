use linux_programming::file_system::print_executable_usage;
use linux_programming::network::dns_resolve;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("host: missing operand");
        print_executable_usage(args[0].as_str(), "$hostname");
        unsafe { libc::exit(libc::EXIT_FAILURE) };
    }
    let _ = dns_resolve(args[1].as_str());
}
