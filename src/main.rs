use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, stdin};
use std::time::Duration;
use serde_json::Value;

mod toutiao;
mod douyin;
mod utils;
mod analysis;


async fn control() -> Result<(), Box<dyn Error>> {
    // 获取命令行参数
    let args = std::env::args();
    // 判断是哪种操作：获取直播间福袋信息｜获取直播间弹幕｜获取视频下载链接｜获取视频
    // println!("获取命令行参数：{args:?}");
    // 没有命令行参数，就打印输出选项
    return if args.len() > 1 {
        println!("有命令行参数: {args:?}");
        Ok(())
    } else {
        // 读取json配置
        let mut root_path = get_path().unwrap();
        root_path.push_str("config.json");
        let file = File::open(root_path)?;
        let reader = BufReader::new(file);
        let config_path: Value = serde_json::from_reader(reader).expect("parse json error");
        // let input = utils::common::select_action();
        println!("config_path: {:?}", config_path);
        let analysis = analysis::webdata::AnalysisData::new(config_path);
        analysis.controller().await?;
        Ok(())
    };
}


fn get_path() -> Result<String, String> {
    // Path to the executable: "/Users/song/Project/my/RustBytes/rust_bytes"
    return match env::current_exe() {
        Ok(path) => {
            println!("Path to the executable: {:?}", path);
            // if src-build in path
            let path_string = path.to_string_lossy().to_string();
            if path_string.contains("rust_bytes") {
                // split end mast translate collect
                let root_path = path_string.split_once("rust_bytes").unwrap();
                println!("root path is:{:?}", root_path);
                Ok(String::from(root_path.0))
            } else {
                Ok(path_string)
            }
        }
        Err(e) => {
            println!("Error getting executable path: {}", e);
            Err(String::from("Error"))
        }
    };
}

#[tokio::main]
async fn main() {
    println!("\n本软件是开源免费软件，仅用于网络数据分析使用，禁止用于一切违法行为！\r
本软件开源地址：https://github.com/TurboWay/bigdata_analyse，
免责声明：该工具仅供学习使用，禁止用于违法行为，是否同意遵守法律法规，一切后果由使用者负责，是否同意？");
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    println!("用户输入的内容是:{}", user_input);
    if user_input.trim() == "是" {
        let res = control().await;
        loop {
            match res {
                Ok(_) => {
                    println!("没有异常，可用于分析数据");
                }
                Err(_) => {
                    println!("发生异常，网络不可访问！等待10秒后重试......");
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            }
        }
    } else {
        println!("立即退出");
    }
}