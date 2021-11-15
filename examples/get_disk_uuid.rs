use linux_programming::file_system::parser::etc;

fn main() {
    dbg!(etc::fstab::parse_etc_fstab());
}

#[cfg(test)]
mod lsblk {
    #[derive(serde::Deserialize)]
    struct LsblkJsonOutput {
        blockdevices: Vec<LsblkItem>,
    }

    #[derive(serde::Deserialize)]
    struct LsblkItem {
        // path: String,
        // fstype: Option<String>,
        uuid: Option<String>,
    }

    #[test]
    fn test() {
        let output = std::process::Command::new("lsblk")
            .arg("--json")
            .arg("--output")
            .arg("uuid")
            .output()
            .unwrap();
        let lsblk_output_str = unsafe { String::from_utf8_unchecked(output.stdout) };
        let lsblk_output = serde_json::from_str::<LsblkJsonOutput>(&lsblk_output_str).unwrap();
        let uuid = lsblk_output
            .blockdevices
            .into_iter()
            .find_map(|x| x.uuid)
            .unwrap_or_default();
        dbg!(uuid);
    }
}
