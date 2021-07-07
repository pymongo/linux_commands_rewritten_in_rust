//! pwd: shell built-in command

fn main() {
    println!(
        "{}",
        linux_commands_rewritten_in_rust::file_system::getcwd()
    );
}
