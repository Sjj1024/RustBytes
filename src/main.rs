mod toutiao;
mod douyin;
mod utils;
mod caicai;


async fn control() {
    // 获取命令行参数
    let args = std::env::args();
    // 判断是哪种操作：获取直播间福袋信息｜获取直播间弹幕｜获取视频下载链接｜获取视频
    // println!("获取命令行参数：{args:?}");
    // 没有命令行参数，就打印输出选项
    if args.len() > 1 {
        println!("有命令行参数: {args:?}");
    } else {
        // let input = utils::common::select_action();
        let input = 8;
        match input {
            1 => {
                // 直播间弹幕
                println!("用户输入的是1");
            }
            2 => {
                // 福袋信息
                println!("用户输入的是2");
                // run lottery main
                // input live url
                let live_url = utils::common::input_url();
                douyin::live::lottery_main(live_url).await;
            }
            3 => {
                // 礼物排行榜
                println!("用户输入3，获取直播间礼物排行榜");
                // input live url
                let live_url = utils::common::input_url();
                douyin::live::live_rank(live_url).await;
            }
            4 => {
                // 测试消息推送
                println!("用户输入4，测试微信消息推送");
                let wx = utils::message::WxPusher::new(String::from("AT_UyFVD4Vhyl7BFUmnicHKrtBI5oz0mY4X"));
                wx.test_push().await.unwrap()
            }
            5 => {
                // 测试消息推送
                println!("用户输入5，定制消息推送");
                let wx = utils::message::WxPusher::new(String::from("AT_UyFVD4Vhyl7BFUmnicHKrtBI5oz0mY4X"));
                wx.push_msg(String::from("你好啊，大佬")).await.unwrap()
            }
            6 => {
                // 测试消息推送
                println!("用户输入6，获取加拿大48的结果");
                let canada = caicai::canada::Canada48::new();
                canada.get_result().await.unwrap();
            }
            7 => {
                // 测试消息推送
                println!("用户输入7，获取加拿大48的预测结果");
                let canada = caicai::canada::Canada48::new();
                canada.get_calculate().await.unwrap();
            }
            8 => {
                // 测试消息推送
                println!("加拿大48主流程控制");
                let canada = caicai::canada::Canada48::new();
                canada.controller().await;
            }
            _ => {
                println!("没有匹配到输入选项");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    control().await;
}