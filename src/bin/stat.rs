//! stat (GNU coreutils)
#![warn(clippy::nursery, clippy::pedantic)]
use linux_commands_rewritten_in_rust::time::format_timestamp_with_nanosecond;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("stat: missing operand");
        eprintln!("Try 'stat --help' for more information.");
        return;
    }
    match args[1].as_str() {
        "--version" => {
            println!("stat (GNU coreutils) rewritten in Rust");
            println!("source code: https://github.com/pymongo/linux_commands_rewritten_in_rust");
        }
        "--help" => {
            println!("help doc is TODO");
        }
        filename => {
            my_stat(filename);
        }
    }
}

fn my_stat(filename: &str) {
    let filename_with_nul = format!("{}\0", filename);
    let mut file_stat = unsafe { std::mem::zeroed() };
    let ret = unsafe { libc::stat(filename_with_nul.as_ptr().cast(), &mut file_stat) };
    if ret == -1 {
        unsafe { libc::perror("\0".as_ptr().cast()) }
    }
    println!("  File: {}", filename);
    println!(
        "  Size: {:<15} Blocks: {:<10} IO Block: {:<6} {}",
        file_stat.st_size,
        file_stat.st_blocks,
        file_stat.st_blksize,
        get_filetype(file_stat.st_mode)
    );
    println!("Device: ");
    println!("Access: ");

    let access_time = format_timestamp_with_nanosecond(file_stat.st_atime, file_stat.st_atime_nsec);
    let modify_time = format_timestamp_with_nanosecond(file_stat.st_mtime, file_stat.st_mtime_nsec);
    println!("Access: {}", access_time);
    println!("Modify: {}", modify_time);
    println!("Change: {}", modify_time);
    // FIXME `/dev/console` create_time should be null
    println!(" Birth: {}", access_time);
}

/**
## 「重要」Unix文件类型

markdown table generate from csv by: https://donatstudios.com/CsvToMarkdownTable

| stat.h   | file_type        | find -type/ls-l(first char) | bash test | example                     | $LS_COLORS |
|----------|------------------|-----------------------------|-----------|-----------------------------|------------|
| S_IFIFO  | FIFO(pipe)       | p                           | -p        | /run/systemd/sessions/1.ref | amber      |
| S_IFCHR  | character device | c                           | -c        | /dev/console                | yellow     |
| S_IFDIR  | directory        | d                           | -d        | /usr/bin/                   | purple     |
| S_IFBLK  | block device     | b                           | -b        | /dev/nvme0n1p2              | yellow     |
| S_IFREG  | regular file     | f/-                         | -f        | /usr/include/stdio.h        | white      |
| S_IFLNK  | symbolic link    | l                           | -L/-h     | /usr/lib/libcurl.so         | aqua       |
| S_IFSOCK | socket           | s                           | -S        | /tmp/mongodb-27017.sock     | magenta    |

```csv
stat.h,file_type,find -type/ls-l(first char),bash test,example,$LS_COLORS
S_IFIFO,FIFO(pipe),p,-p,/run/systemd/sessions/1.ref,amber
S_IFCHR,character device,c,-c,/dev/console,yellow
S_IFDIR,directory,d,-d,/usr/bin/,purple
S_IFBLK,block device,b,-b,/dev/nvme0n1p2,yellow
S_IFREG,regular file,f/-,-f,/usr/include/stdio.h,white
S_IFLNK,symbolic link,l,-L/-h,/usr/lib/libcurl.so,aqua
S_IFSOCK,socket,s,-S,/tmp/mongodb-27017.sock ,magenta
```

- 串行读写设备示例: 磁带，键盘
- 块状读写设备示例: DVD/CD, HDD 都是一次读写一个扇区
*/
#[allow(clippy::doc_markdown)]
fn get_filetype<'a>(st_mode: u32) -> &'a str {
    let filetype_mask = st_mode & libc::S_IFMT;
    match filetype_mask {
        libc::S_IFIFO => "FIFO",
        libc::S_IFCHR => "character device",
        libc::S_IFDIR => "directory",
        libc::S_IFBLK => "block device",
        libc::S_IFREG => "regular file",
        libc::S_IFLNK => "symbolic link",
        libc::S_IFSOCK => "socket",
        _ => unreachable!(),
    }
}

#[test]
fn test_stat_st_mode_bit_mask() {
    // 1000 000 110100100
    // kind     permission
    println!("{:016b}", libc::S_IFMT);

    println!("{:016b}", libc::S_ISUID);
    println!("{:016b}", libc::S_ISGID);
    println!("{:016b}", libc::S_ISVTX);

    println!("{:016b}", libc::S_IRWXU);
    println!("{:016b}", libc::S_IRWXG);
    println!("{:016b}", libc::S_IRWXO);
}
