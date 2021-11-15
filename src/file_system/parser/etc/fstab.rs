pub fn parse_etc_fstab() {
    for line in std::fs::read_to_string("/etc/fstab").unwrap().lines() {
        if line.starts_with('#') {
            continue;
        }
        let row = line.split_whitespace().collect::<Vec<_>>();
        let mount_point = row[1];
        if mount_point == "/" {
            let uuid = row[0].split_once('=').unwrap().1;
            dbg!(uuid);
        }
    }
}

#[test]
fn test_parse_etc_fstab() {
    parse_etc_fstab();
}

#[cfg(test)]
mod parse_with_error_handling {
    fn parse_etc_fstab_with_error_handling() -> Option<String> {
        for line in std::fs::read_to_string("/etc/fstab").ok()?.lines() {
            if line.starts_with('#') {
                continue;
            }
            let row = line.split_whitespace().collect::<Vec<_>>();
            let mount_point = row[1];
            if mount_point == "/" {
                let uuid = row[0].split_once('=')?.1;
                return Some(uuid.to_string());
            }
        }
        None
    }

    #[test]
    fn test_parse_etc_fstab_with_error_handling() {
        parse_etc_fstab_with_error_handling();
    }
}
