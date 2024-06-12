use std::io::stdin;

pub enum Select {
    Input(i32)
}

pub fn select_action() -> i32 {
    println!("
请选择要执行的操作:
1.抖音直播间弹幕礼物抓取
2.抖音直播间福袋信息抓取
3.抖音直播间礼物排行榜
4.测试微信消息推送
5.定制微信消息推送
6.获取加拿大48的结果
7.获取加拿大48的预测结果
8.运行加拿大48主流程程序
请输入:");
    // wait use select
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    let input_num: i32 = user_input.trim().parse().unwrap();
    println!("用户的输入是:{input_num:?}");
    return input_num;
}


// 请输入直播间地址或者用户首页地址
pub fn input_url() -> String {
    println!("请输入直播间地址:");
    let mut live_url = String::new();
    stdin().read_line(&mut live_url).unwrap();
    return live_url.trim().to_string();
}