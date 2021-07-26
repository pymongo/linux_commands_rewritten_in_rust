use linux_commands_rewritten_in_rust::network::{dns_lookup_gethostbyname, in_addr_to_string};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("stat: missing operand");
        eprintln!("Try 'nslookup --help' for more information.");
        return;
    }

    let hostname = args[1].as_str();
    println!("{}", hostname);
    let addr_list = dns_lookup_gethostbyname(hostname);
    for addr in addr_list {
        println!("Address: {}", in_addr_to_string(addr));
    }
}
