use sysinfo::Disk;

use crate::pcstatus::DiskSpace;

use std::error::Error;

pub struct NoticeInfo<'a> {
    /// ディスク容量情報
    pub disk_spaces: &'a [DiskSpace],
    /// アラートを出す残り容量閾値(%)
    pub remaining_space_alert: u32,
}

pub struct NotifyedInfo {
    pub general: bool,
    pub low_remaining_disk_space: bool,
    pub btrfs_scrub_error: bool,
}

pub fn notify(info: &NoticeInfo) -> Result<NotifyedInfo, Box<dyn Error>> {
    let mut result = NotifyedInfo {
        general: false,
        low_remaining_disk_space: false,
        btrfs_scrub_error: false,
    };
    match notify_general(info) {
        Ok(_) => result.general = true,
        Err(e) => {
            eprintln!("Failed to notify general: {}", e);
            result.general = false;
        }
    };

    let low_remining_disk =
        check_remaining_disk_size(info.disk_spaces, info.remaining_space_alert)?;

    todo!()
}

fn notify_general(info: &NoticeInfo) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn check_remaining_disk_size(
    disk_space: &[DiskSpace],
    remaining_space_alert: u32,
) -> Result<Vec<DiskSpace>, Box<dyn Error>> {
    let mut low_remaining_disk = Vec::new();

    for ds in disk_space {
        let used = ds.total - ds.available_space;
        let used_percent = (used as f64 / ds.total as f64) * 100.0;

        if used_percent >= 100.0 - remaining_space_alert as f64 {
            low_remaining_disk.push(ds);
        }
    }

    Ok(low_remaining_disk)
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;

    #[test]
    fn test_notify_slack() {
        let info = NoticeInfo {
            disk_spaces: &[DiskSpace {
                mount_point: path::PathBuf::from("C:\\"),
                total: 100,
                available_space: 50,
            }],
            remaining_space_alert: 10,
        };

        let notifyed = notify(&info).unwrap();
        assert!(notifyed.general);
        assert!(notifyed.low_remaining_disk_space);
        assert!(!notifyed.btrfs_scrub_error);
    }
}
