#![warn(clippy::nursery, clippy::pedantic)]
fn main() {
    println!(
        "{}",
        linux_commands_rewritten_in_rust::file_system::getcwd()
    );
}
