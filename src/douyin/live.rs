use crate::douyin::apis::DouYinReq;

pub async fn lottery_main() {
    println!("获取抖音直播间福袋的主逻辑");
    // 先创建一个抖音直播间请求结构体：包含请求对象和一些请求api
    let live_url = String::from("https://live.douyin.com/972176515698");
    let mut live_req = DouYinReq::new(live_url);
    // 执行搜索请求的api
    live_req.get_room_id().await.unwrap();
    // 获取直播间福袋信息
    live_req.get_lottery_info().await;
}