use std::error::Error;
use std::time::Duration;

mod toutiao;
mod douyin;
mod utils;
mod caicai;


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
        // let input = utils::common::select_action();
        let input = 8;
        match input {
            1 => {
                // 直播间弹幕
                println!("用户输入的是1");
                Ok(())
            }
            2 => {
                // 福袋信息
                println!("用户输入的是2");
                // run lottery main
                // input live url
                let live_url = utils::common::input_url();
                douyin::live::lottery_main(live_url).await;
                Ok(())
            }
            3 => {
                // 礼物排行榜
                println!("用户输入3，获取直播间礼物排行榜");
                // input live url
                let live_url = utils::common::input_url();
                douyin::live::live_rank(live_url).await;
                Ok(())
            }
            4 => {
                // 测试消息推送
                println!("用户输入4，测试微信消息推送");
                let wx = utils::message::WxPusher::new(String::from("AT_UyFVD4Vhyl7BFUmnicHKrtBI5oz0mY4X"));
                wx.test_push().await?;
                Ok(())
            }
            5 => {
                // 测试消息推送
                println!("用户输入5，定制消息推送");
                let wx = utils::message::WxPusher::new(String::from("AT_UyFVD4Vhyl7BFUmnicHKrtBI5oz0mY4X"));
                wx.push_msg(String::from("你好啊，大佬")).await?;
                Ok(())
            }
            6 => {
                // 测试消息推送
                println!("用户输入6，获取加拿大48的结果");
                let canada = caicai::canada::Canada48::new();
                canada.get_result().await?;
                Ok(())
            }
            7 => {
                // 测试消息推送
                println!("用户输入7，获取加拿大48的预测结果");
                let canada = caicai::canada::Canada48::new();
                canada.get_calculate().await?;
                Ok(())
            }
            8 => {
                // 测试消息推送
                println!("加拿大48主流程控制");
                let canada = caicai::canada::Canada48::new();
                canada.controller().await?;
                Ok(())
            }
            _ => {
                println!("没有匹配到输入选项");
                Ok(())
            }
        }
    };
}

#[tokio::main]
async fn main() {
    loop {
        let res = control().await;
        match res {
            Ok(_) => {
                println!("没有异常");
            }
            Err(_) => {
                println!("发生异常，等待10秒后重试......");
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }
}