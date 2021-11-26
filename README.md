# Linux commands rewritten in Rust

## project structure

- src/bin: Linux commands rewritten in Rust
- src/database: like sqlx project, include database adapters eg. MySQL
- src/dylibs_binding: Rust binding for eg. libmysqlclient.so, libsqlite3.so
- src/file_system: some file_system relative bindings e.g. `basename()`
- src/file_system/parser: parse to some files e.g. `/proc/net/route` to get Linux system information from files
- src/network: network API which libc doesn't include, eg. inet_aton, gethostbyname
- src/time: time API which libc doesn't include, eg. strftime, strptime
- examples: C/C++/Rust SIGABRT/SIGSEGV bad examples and how to fix tips
- docs: documents or notes called by eg. `#![doc = include_str!("README.md”)]`

## cargo test must run in **single thread**

To run database test you need to copy config file and edit it(eg. your mysql password):

> cp database_config.toml.example database_config.toml && vim database_config.toml

this config is only for mysql testing, run commands in src/bin doesn't need this

because multi database adapters test is using a **same file** to store data

> RUST_TEST_THREADS=1 cargo test

or 

> cargo test -- --test-threads=1

## known bugs on target armv7-unknown-linux-gnueabihf

- database::adapters::dbm  may double-free or malloc corrupted

## reference:
- [gnu core utils rewritten in Rust](https://github.com/uutils/coreutils)
- <https://gitlab.redox-os.org/redox-os/relibc>
- <https://zaiste.net/posts/shell-commands-rust/>
