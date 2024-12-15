mod notice;
mod pcstatus;

use clap::Parser;
use notice::NoticeInfo;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, help = "設定ファイルパス")]
    config_file_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    /// ディスク容量を取得するマウントポイント
    mount_points: Vec<String>,
    /// 残り容量アラート閾値(%指定)
    remaining_space_alert: u32,
}

fn main() {
    let args = Args::parse();

    let config = parse_config(&args.config_file_path).unwrap();

    println!("{:?}", config);

    let mut mount_points = Vec::new();
    for mp in &config.mount_points {
        mount_points.push(path::Path::new(mp));
    }

    let disk_spaces = pcstatus::disk_space(&mount_points).unwrap();

    println!("{:?}", disk_spaces);

    notice::notify(&NoticeInfo {
        disk_spaces: &disk_spaces,
        remaining_space_alert: config.remaining_space_alert,
    })
    .unwrap();
}

fn parse_config(config_file_path: &str) -> Result<Config, Box<dyn Error>> {
    let f = File::open(config_file_path)?;
    let config = serde_json::from_reader(f)?;
    Ok(config)
}
