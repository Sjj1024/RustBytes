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
        let input = utils::common::select_action();
        // input live url
        let live_url = utils::common::input_url();
        match input {
            1 => {
                // 直播间弹幕
                println!("用户输入的是1");
            }
            2 => {
                // 福袋信息
                println!("用户输入的是2");
                // run lottery main
                douyin::live::lottery_main(live_url).await;
            }
            3 =>{
                // 礼物排行榜
                println!("用户输入3，获取直播间礼物排行榜");
                douyin::live::live_rank(live_url).await;
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