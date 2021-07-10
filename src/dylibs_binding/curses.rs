#[cfg(test)]
use std::os::raw::c_int;

/// curses/ncurses: a terminal ui facilities
#[cfg(test)]
#[link(name = "curses")]
extern "C" {
    type window;
    fn initscr() -> *mut window;
    fn endwin() -> c_int;
    fn refresh() -> c_int;
    #[link_name = "move"]
    fn move_(x: c_int, y: c_int) -> c_int;
    fn printw(format: *const libc::c_char, ...) -> c_int;
}

#[cfg(test)]
unsafe fn curses_hello_world() {
    initscr();
    move_(10, 15);
    printw("Hello World\0".as_ptr().cast());
    refresh();
    libc::sleep(2);
    endwin();
    libc::exit(libc::EXIT_SUCCESS);
}

#[test]
fn run_curses_hello_world() {
    unsafe {
        curses_hello_world();
    }
}
