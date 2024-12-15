use std::{
    error::Error,
    path::{self, Path},
};
use sysinfo::Disks;

/// ディスク容量取得結果
#[derive(Debug)]
pub struct DiskSpace {
    pub mount_point: path::PathBuf,
    pub total: u64,
    pub available_space: u64,
}

pub fn disk_space(mount_points: &[&Path]) -> Result<Vec<DiskSpace>, Box<dyn Error>> {
    let disks = Disks::new_with_refreshed_list();

    let mut disk_spaces = Vec::new();

    for disk in disks.list() {
        println!("{:?}", disk);

        if mount_points.iter().any(|x| *x == disk.mount_point()) {
            let ds = DiskSpace {
                mount_point: disk.mount_point().to_path_buf(),
                total: disk.total_space(),
                available_space: disk.available_space(),
            };

            disk_spaces.push(ds);
        }
    }

    Ok(disk_spaces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_space() {
        let mount_points;
        if cfg!(target_os = "windows") {
            mount_points = vec![Path::new("C:\\")];
        } else if cfg!(target_os = "linux") {
            mount_points = vec![Path::new("/home")];
        } else {
            panic!("Unsupported OS");
        }
        let disks = disk_space(&mount_points).unwrap();

        assert_eq!(disks.len(), 1);
    }
}
