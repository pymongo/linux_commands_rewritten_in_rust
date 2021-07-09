/*!
SIGABRT 的可能原因:
- double free, example: `closedir(dirp);closedir(dirp);`

## 避免 double free 的编码习惯

单线程应用，在 free(ptr) 之后将 ptr 设置为 NULL，多线程应用则用引用计数

```no_run
// bad
let dirp = libc::opendir("/home\0".as_ptr().cast());
libc::closedir(dirp);
libc::closedir(dirp);
```

```no_run
// good example
let mut dirp = libc::opendir("/home\0".as_ptr().cast());
libc::closedir(dirp);
dirp = std::ptr::null_mut();
libc::closedir(dirp);
dirp = std::ptr::null_mut();
```

## double free 不一定及时报错

《 Beginning Linux Programm 4th edition 》 Page 260 有详细介绍(PDF 293页):

> allocated memory is writing beyond the end of an allocated block(one example is double-free)

例如尝试free一块已经回收的内存(allocated memory beyond the block)

> one reason malloc failed is the memory structures have been corrupted, When this happens, the program may not terminate immediately

例如错误递归或循环间隐式的free同一个资源，程序会被valgrind检查出`closedir InvalidFree`

但是进程却能正常退出，如果加上 current_dir 的函数调用程序则 SIGABRT

原因是 操作系统/进程 并不会立即回收内存，更像是异步的回收内存，在回收/申请堆内存时才会检查并报错 SIGABRT

而 Rust 的 current_dir 就像 Future::poll 去申请内存(因为文件绝对路径可能很长，需要扩容)

Rust 申请内存时发现当前进程居然有几块 double free 的内存，为了避免错误进一步扩散，就报错 SIGABRT
*/
fn main() {
    let dir_name = "/\0";
    let dir_name_cstr = dir_name.as_ptr().cast();
    let dirp = unsafe { libc::opendir(dir_name_cstr) };
    if dirp.is_null() {
        unsafe {
            libc::perror(dir_name_cstr);
        }
        return;
    }
    unsafe { libc::chdir(dir_name_cstr) };
    unsafe {
        traverse_dir_dfs(dirp, 0);
    }
}

unsafe fn traverse_dir_dfs(dirp: *mut libc::DIR, indent: usize) {
    loop {
        let dir_entry = libc::readdir(dirp);
        if dir_entry.is_null() {
            // malloc(): unsorted double linked list corrupted\n `Aborted (core dumped)` exit code 134 (interrupted by signal 6: SIGABRT)
            let _sigabrt_line = std::env::current_dir().unwrap();
            return;
        }
        let dir_entry = *dir_entry;
        let filename_cstr = dir_entry.d_name.as_ptr();

        // skip current directory and parent directory
        if libc::strcmp(filename_cstr, ".\0".as_ptr().cast()) == 0
            || libc::strcmp(filename_cstr, "..\0".as_ptr().cast()) == 0
        {
            continue;
        }

        // check file whether a directory
        let mut stat_buf = std::mem::zeroed();
        let stat_ret = libc::lstat(filename_cstr, &mut stat_buf); // lstat doesn't follow link
        if stat_ret == -1 {
            panic!("{}", std::io::Error::last_os_error());
        }
        let is_dir = stat_buf.st_mode & libc::S_IFMT == libc::S_IFDIR;

        // convert filename from [c_char; 256] to String
        let filename_len = libc::strlen(filename_cstr);
        let filename_bytes = &*(&dir_entry.d_name[..filename_len] as *const [i8] as *const [u8]);
        let filename_string = String::from_utf8_unchecked(filename_bytes.to_owned());
        println!(
            "{}{}{}",
            " ".repeat(indent),
            filename_string,
            if is_dir { "/" } else { "" }
        );

        if is_dir {
            let dirp_inner_dir = libc::opendir(filename_cstr);
            libc::chdir(filename_cstr);
            traverse_dir_dfs(dirp_inner_dir, indent + 4);
            libc::chdir("..\0".as_ptr().cast());
            // Bug is here: this should be `closedir(dirp_inner_dir)`
            // https://github.com/rust-lang/rust/issues/86899
            // if a directory has two subdirectory, this would cause **`double free`** (closedir to a same dir twice)
            libc::closedir(dirp);
        }
    }
}
