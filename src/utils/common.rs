use std::io::stdin;

pub enum Select {
    Input(i32)
}

pub fn select_action() -> i32 {
    println!("请输入要分析的网站地址:");
    // wait use select
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    println!("用户输入的是:{user_input:?}");
    return 8;
}


// 请输入直播间地址或者用户首页地址
pub fn input_url() -> String {
    println!("请输入直播间地址:");
    let mut live_url = String::new();
    stdin().read_line(&mut live_url).unwrap();
    return live_url.trim().to_string();
}