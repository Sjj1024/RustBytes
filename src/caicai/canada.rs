use std::io::{self, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use reqwest::Client;
use serde_json::Value;
use crate::utils::message::WxPusher;

pub struct Canada48 {
    request: Client,
    base_url: String,
    page_size: i32,
    wx_pusher: WxPusher,
}

impl Canada48 {
    pub fn new() -> Self {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        let page_size = 10;
        // 推送的apptoken
        let weixin = WxPusher::new(String::from("AT_UyFVD4Vhyl7BFUmnicHKrtBI5oz0mY4X"));
        return Canada48 { request: client, base_url: String::from("http://23.225.7.133:828"), page_size, wx_pusher: weixin };
    }

    // 主流程控制
    pub async fn controller(&self) -> Result<(), Box<dyn std::error::Error>> {
        // println!("主流程控制");
        // loop循环
        loop {
            // 获取开奖结果和预测结果数据
            self.get_result().await?;
            self.get_calculate().await?;
            // 再等待5秒后开始循环
            println!("开始获取下次开奖时间......");
            // 下次开奖section
            let mut section: u64 = 0;
            // 获取下次开奖时间，如果超过30分钟，等待10秒钟重新获取
            let mut next_second = Duration::from_millis(1);
            for _ in 1..10 {
                tokio::time::sleep(Duration::from_secs(10)).await;
                (next_second, section) = self.get_next().await?;
                let inner_duration = next_second.as_secs();
                // 如果开奖时间超过30分钟，发送消息报错
                let minutes_last = inner_duration / 60;
                if minutes_last > 30 {
                    continue;
                } else {
                    break;
                }
            }
            // let next_second = self.get_next().await.unwrap();
            // 控制台刷新还剩多少秒,等待一段时间，模拟耗时操作
            self.flush_second(&next_second).await?;
            println!("\n开始获取最新结果数据......");
            // 检测下次开奖section是否已经开奖，已经开奖才执行后续逻辑，否则持续检测
            for _ in 1..10 {
                if self.check_section(&section).await? {
                    println!("继续检测开奖结果...");
                } else {
                    break;
                }
            }
            // 要多等待几秒钟才可以向服务器发送最新数据，否则获取不到最新数据
            tokio::time::sleep(Duration::from_millis(7000)).await;
        }
    }

    // 持续检测section是否已经开奖
    pub async fn check_section(&self, section: &u64) -> Result<bool, Box<dyn std::error::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "*/*".parse()?);
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8".parse()?);
        headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse()?);
        headers.insert("Origin", "http://23.225.7.133:828".parse()?);
        headers.insert("Proxy-Connection", "keep-alive".parse()?);
        headers.insert("Referer", "http://23.225.7.133:828/".parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        let data = format!("section={}", section);
        let request = self.request.post("http://23.225.7.133:828/Mobile/Indexs/open/type/2")
            .headers(headers)
            .body(data);
        let response = request.send().await?;
        let body = response.text().await?;
        println!("检测开奖结果: {}", body);
        return if body.contains(r#""""#) {
            Ok(true)
        } else {
            Ok(false)
        };
    }

    // 控制台倒计时
    pub async fn flush_second(&self, duration: &Duration) -> Result<(), Box<dyn std::error::Error>> {
        // 输出相差的分钟和秒数
        let mut inner_duration = duration.as_secs();
        // 如果开奖时间超过30分钟，发送消息报错
        let minutes_last = inner_duration / 60;
        // 如果分钟超过30，就说明异常，发送微信通知
        if minutes_last > 30 {
            println!("下次开奖超30分钟，可能是发生异常了");
            self.wx_pusher.push_summary(String::from("下次开奖超30分钟"), String::from("下次开奖超30分钟，可能是发生异常了")).await.unwrap();
        }
        loop {
            // 等待一段时间，模拟耗时操作
            tokio::time::sleep(Duration::from_secs(1)).await;
            inner_duration -= 1;
            let minutes = inner_duration / 60;
            let seconds = inner_duration % 60;
            let format_second = if seconds < 10 { format!("0{seconds:?}") } else { format!("{seconds:?}") };
            print!("\r距离下次开奖还剩: {minutes:?}分{format_second}秒");
            io::stdout().flush().unwrap();
            if inner_duration - 1 <= 0 {
                return Ok(());
            }
        }
    }


    // 获取下次开奖时间
    pub async fn get_next(&self) -> Result<(Duration, u64), Box<dyn std::error::Error>> {
        // println!("获取下次开奖时间......");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json, text/javascript, */*; q=0.01".parse()?);
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8".parse()?);
        headers.insert("Connection", "keep-alive".parse()?);
        // headers.insert("Cookie", "_pk_id.1.573e=1f28b480ae3ce652.1718096527.; cookieconsent_status=dismiss; _pk_ref.1.573e=%5B%22%22%2C%22%22%2C1718104584%2C%22https%3A%2F%2Fmp.csdn.net%2Fmp_blog%2Fcreation%2Feditor%2F139578386%22%5D; _pk_ses.1.573e=1".parse()?);
        headers.insert("Referer", "http://23.225.7.133:828/".parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        let request = self.request.get("http://23.225.7.133:828/Mobile/Indexs/next/type/2")
            .headers(headers);
        let response = request.send().await?;
        let body = response.text().await?;
        if body.contains("openTime") {
            let map_value: Value = serde_json::from_str(&body).unwrap();
            // 获取下次开奖时间
            let open_time_s = map_value.get("openTime_s").unwrap().as_str().unwrap();
            let open_time = map_value.get("openTime").unwrap().as_u64().unwrap();
            let server_time = map_value.get("serverTime").unwrap().as_u64().unwrap();
            let section = map_value.get("section").unwrap().as_u64().unwrap();
            // 本地当前时间戳
            let local_time = SystemTime::now();
            let since_epoch = local_time.duration_since(UNIX_EPOCH).unwrap().as_millis();
            // 如果服务器时间<开奖时间，就用服务器时间，否则使用本地时间
            let since_millis: u64;
            // 有可能是开奖时间小于服务器时间和本地时间，说明开奖时间有问题
            if server_time < open_time {
                since_millis = open_time - server_time;
            } else if (since_epoch as u64) < open_time {
                since_millis = open_time.wrapping_sub(since_epoch as u64);
            } else {
                // 说明开奖时间有问题了，暂时没有好的解决办法
                since_millis = open_time.wrapping_sub(since_epoch as u64);
            }
            // 还剩多少毫秒
            let duration = Duration::from_millis(since_millis);
            println!("下次开奖时间是: {open_time_s}");
            println!("开奖时间戳: {open_time}");
            println!("服务器时间: {server_time}");
            println!("本地器时间: {since_epoch}");
            println!("开奖剩余时间: {}秒", duration.as_secs());
            return Ok((duration, section));
        } else {
            println!("返回json数据异常:{body}");
            panic!("需要开代理才可以访问网站！");
        }
    }

    // 获取开奖结果
    pub async fn get_result(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("获取开奖结果......");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json, text/javascript, */*; q=0.01".parse()?);
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8".parse()?);
        headers.insert("Proxy-Connection", "keep-alive".parse()?);
        headers.insert("Referer", "http://23.225.7.133:828/".parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        let request = self.request.get(format!("{}/Mobile/Indexs/myOpens/type/2/pz/{}/page/1", self.base_url, self.page_size))
            .headers(headers);
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        // 将文本字符串转为结构体: 会被防火墙截胡,导致返回的防诈骗广告
        if body.contains("total") {
            let map_value: Value = serde_json::from_str(&body).unwrap();
            self.handle_result(&map_value).await?;
            Ok(())
        } else {
            println!("返回json数据异常:{body}");
            panic!("需要开代理才可以访问网站！");
        }
    }

    // 获取预测结果
    pub async fn get_calculate(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("获取预测结果......");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json, text/javascript, */*; q=0.01".parse()?);
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8".parse()?);
        headers.insert("Proxy-Connection", "keep-alive".parse()?);
        headers.insert("Referer", "http://23.225.7.133:828/".parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        let request = self.request.get(format!("{}/Mobile/Indexs/yuce/type/2/pz/{}/1", self.base_url, self.page_size))
            .headers(headers);
        let response = request.send().await?;
        let body = response.text().await?;
        // 将文本字符串转为结构体
        if body.contains("total") {
            let map_value: Value = serde_json::from_str(&body).unwrap();
            self.handle_calculate(&map_value).await?;
            Ok(())
        } else {
            println!("返回json数据异常:{body}");
            panic!("需要开代理才可以访问网站！");
        }
    }

    // 处理结果数据是否发送微信通知
    pub async fn handle_result(&self, result: &Value) -> Result<(), Box<dyn std::error::Error>> {
        let res_data = result.get("data").unwrap().as_array().unwrap();
        // println!("处理开奖结果: {res_data:?}");
        let last_four = &res_data[0..4];
        // println!("获取最新的四条结果数据是: {last_four:?}");
        let mut open_nums: Vec<&str> = vec![];
        let mut single_dou: Vec<&str> = vec![];
        for data in last_four.iter() {
            let num = data.get("openNum").unwrap().as_i64().unwrap();
            // 判断大小
            if num <= 13 {
                open_nums.push("小");
            } else {
                open_nums.push("大");
            }
            // 判断单双
            if num % 2 == 0 {
                println!("双数：{num:?}");
                single_dou.push("双");
            } else {
                println!("单数：{num:?}");
                single_dou.push("单");
            }
        }
        println!("开奖的四个结果大小是: {open_nums:?} 单双结果是：{single_dou:?}");
        // 判断大小
        if open_nums == vec!["大", "大", "大", "大"] {
            println!("四个大");
            self.wx_pusher.push_summary(String::from("四个大"), String::from("开奖结果: 四个大")).await?;
        } else if open_nums == vec!["小", "小", "小", "小"] {
            println!("四个小");
            self.wx_pusher.push_summary(String::from("四个小"), String::from("开奖结果: 四个小")).await?;
        } else if open_nums == vec!["小", "大", "小", "大"] {
            println!("小大小大");
            self.wx_pusher.push_summary(String::from("小大小大"), String::from("开奖结果: 小大小大")).await?;
        } else if open_nums == vec!["大", "小", "大", "小"] {
            println!("大小大小");
            self.wx_pusher.push_summary(String::from("大小大小"), String::from("开奖结果: 大小大小")).await?;
        } else {
            println!("不用关心的大小结果");
        }
        // 判断单双
        if single_dou == vec!["单", "单", "单", "单"] {
            println!("四个单");
            self.wx_pusher.push_summary(String::from("四个单"), String::from("开奖结果: 四个单")).await?;
        } else if single_dou == vec!["双", "双", "双", "双"] {
            println!("四个双");
            self.wx_pusher.push_summary(String::from("四个双"), String::from("开奖结果: 四个双")).await?;
        } else if single_dou == vec!["单", "双", "单", "双"] {
            println!("单双单双");
            self.wx_pusher.push_summary(String::from("单双单双"), String::from("开奖结果: 单双单双")).await?;
        } else if single_dou == vec!["双", "单", "双", "单"] {
            println!("双单双单");
            self.wx_pusher.push_summary(String::from("双单双单"), String::from("开奖结果: 双单双单")).await?;
        } else {
            println!("不用关心的单双结果");
        }
        Ok(())
    }


    // 处理预测结果是否发送微信通知
    pub async fn handle_calculate(&self, result: &Value) -> Result<(), Box<dyn std::error::Error>> {
        let res_data = result.get("data").unwrap().as_array().unwrap();
        let four_data = &res_data[1..5];
        // println!("获取最新的四条预测数据是: {four_data:?}");
        let mut left_color: Vec<&str> = vec![];
        let mut right_color: Vec<&str> = vec![];
        for data in four_data.iter() {
            let r1_r = data.get("r1_r").unwrap().as_i64().unwrap();
            let r2_r = data.get("r2_r").unwrap().as_i64().unwrap();
            // 判断左边颜色：0蓝 1红
            if r1_r == 0 {
                left_color.push("蓝");
            } else {
                left_color.push("红");
            }
            // 判断右边颜色：0蓝 1红
            if r2_r == 0 {
                right_color.push("蓝");
            } else {
                right_color.push("红");
            }
        }
        println!("预测的四个结果是: {left_color:?} 右侧: {right_color:?}");
        // 判断左边颜色
        if left_color == vec!["蓝", "蓝", "蓝", "蓝"] {
            println!("左侧预测: 四个蓝");
            self.wx_pusher.push_summary(String::from("左侧四个蓝"), String::from("预测结果: 左侧四个蓝")).await?;
        } else if left_color == vec!["红", "红", "红", "红"] {
            println!("左侧四个红");
            self.wx_pusher.push_summary(String::from("左侧四个红"), String::from("预测结果: 左侧四个红")).await?;
        } else if left_color == vec!["蓝", "红", "蓝", "红"] {
            println!("左侧蓝红蓝红");
            self.wx_pusher.push_summary(String::from("左侧蓝红蓝红"), String::from("预测结果: 左侧蓝红蓝红")).await?;
        } else if left_color == vec!["红", "蓝", "红", "蓝"] {
            println!("左侧红蓝红蓝");
            self.wx_pusher.push_summary(String::from("左侧红蓝红蓝"), String::from("预测结果: 左侧红蓝红蓝")).await?;
        } else {
            println!("左侧不用关心的结果");
        }
        // 判断右边颜色
        if right_color == vec!["蓝", "蓝", "蓝", "蓝"] {
            println!("右侧四个蓝");
            self.wx_pusher.push_summary(String::from("右侧四个蓝"), String::from("预测结果: 右侧四个蓝")).await?;
        } else if right_color == vec!["红", "红", "红", "红"] {
            println!("右侧四个红");
            self.wx_pusher.push_summary(String::from("右侧四个红"), String::from("预测结果: 右侧四个红")).await?;
        } else if right_color == vec!["蓝", "红", "蓝", "红"] {
            println!("右侧蓝红蓝红");
            self.wx_pusher.push_summary(String::from("右侧蓝红蓝红"), String::from("预测结果: 右侧蓝红蓝红")).await?;
        } else if right_color == vec!["红", "蓝", "红", "蓝"] {
            println!("右侧红蓝红蓝");
            self.wx_pusher.push_summary(String::from("右侧红蓝红蓝"), String::from("预测结果: 右侧红蓝红蓝")).await?;
        } else {
            println!("右侧不用关心的结果");
        }
        Ok(())
    }
}