use std::error::Error;
use crate::douyin::apis::DouYinReq;


// 获取直播间信息
pub async fn live_main() {
    println!("获取直播间弹幕信息等")
    //
}


// 中间控制层
pub async fn lottery_main(url: String) {
    println!("获取抖音直播间福袋的主逻辑");
    // 先创建一个抖音直播间请求结构体：包含请求对象和一些请求api
    // let live_url = String::from("https://live.douyin.com/926054037870");
    let mut live_req = DouYinReq::new(url);
    // 获取直播间room_id
    let res = live_req.get_room_id().await;
    match res {
        Ok(_) => {
            println!("success");
        }
        Err(_) => {
            println!("error");
        }
    }
    // 获取直播间福袋信息
    live_req.get_lottery_info().await.expect("TODO: panic message");
}


// 直播间礼物排行榜
pub async fn live_rank(url: String) {
    println!("直播间礼物排行榜:");
    // get live room id
    let mut live_req = DouYinReq::new(url);
    live_req.get_room_id().await.unwrap();
    live_req.get_rank_info().await.unwrap();
}