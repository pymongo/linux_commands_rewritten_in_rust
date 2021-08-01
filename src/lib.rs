#![feature(extern_types)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::doc_markdown,
    clippy::missing_safety_doc,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
#![doc=include_str!("../README.md")]
#![doc=include_str!("../docs/system_call_notes.md")]

#[cfg(test)]
pub mod database;
pub mod dylibs_binding;
pub mod errno;
pub mod file_system;
mod macros;
pub mod network;
pub mod time;

// #include <linux/limits.h>
pub const NAME_MAX: usize = 256;
pub const SOCKADDR_IN_LEN: libc::socklen_t =
    std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
