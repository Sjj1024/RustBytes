use std::error::Error;
use std::io::{self, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use reqwest::Client;
use serde_json::Value;
use crate::utils::message::WxPusher;

pub struct AnalysisData {
    request: Client,
    base_url: String,
    page_size: i32,
    wx_pusher: WxPusher,
}

impl AnalysisData {
    pub fn new(config: Value) -> Self {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        let page_size = 10;
        // 推送的apptoken
        let apptoken = config.get("appToken").expect("get url error").as_str().unwrap();
        let users = config.get("userIds").unwrap();
        let weixin = WxPusher::new(String::from(apptoken), users);
        // 获取地址：
        let url = config.get("url").expect("get url error").as_str().unwrap();
        return AnalysisData { request: client, base_url: String::from(url), page_size, wx_pusher: weixin };
    }

    // 主流程控制
    pub async fn controller(&self) -> Result<(), Box<dyn Error>> {
        // // println!("主流程控制");
        // loop循环
        loop {
            // 获取data和盲猜数据
            self.get_result().await?;
            self.get_calculate().await?;
            // 再等待5秒后开始循环
            // println!("start next handle......");
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
            // println!("\nstart new data......");
            // 检测下次开奖section是否已经开奖，已经开奖才执行后续逻辑，否则持续检测
            for _ in 1..10 {
                if self.check_section(&section).await? {
                    // println!("continue...");
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
        headers.insert("Origin", self.base_url.parse()?);
        headers.insert("Proxy-Connection", "keep-alive".parse()?);
        headers.insert("Referer", self.base_url.parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        let data = format!("section={}", section);
        let request = self.request.post(format!("{}/Mobile/Indexs/open/type/2", self.base_url))
            .headers(headers)
            .body(data);
        let response = request.send().await?;
        let body = response.text().await?;
        // println!("get data: {}", body);
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
            // println!("next time more three zero, may be error");
            self.wx_pusher.push_summary(&String::from("下次开奖超30分钟"), String::from("下次开奖超30分钟，可能是发生异常了")).await.unwrap();
        }
        loop {
            // 等待一段时间，模拟耗时操作
            tokio::time::sleep(Duration::from_secs(1)).await;
            inner_duration -= 1;
            let minutes = inner_duration / 60;
            let seconds = inner_duration % 60;
            let format_second = if seconds < 10 { format!("0{seconds:?}") } else { format!("{seconds:?}") };
            // print!("\rnext time: {minutes:?}m{format_second}s");
            // print!("\r{inner_duration}");
            io::stdout().flush().unwrap();
            if inner_duration - 1 <= 0 {
                return Ok(());
            }
        }
    }


    // 获取下次开奖时间
    pub async fn get_next(&self) -> Result<(Duration, u64), Box<dyn Error>> {
        // // println!("获取下次开奖时间......");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json, text/javascript, */*; q=0.01".parse()?);
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8".parse()?);
        headers.insert("Connection", "keep-alive".parse()?);
        // headers.insert("Cookie", "_pk_id.1.573e=1f28b480ae3ce652.1718096527.; cookieconsent_status=dismiss; _pk_ref.1.573e=%5B%22%22%2C%22%22%2C1718104584%2C%22https%3A%2F%2Fmp.csdn.net%2Fmp_blog%2Fcreation%2Feditor%2F139578386%22%5D; _pk_ses.1.573e=1".parse()?);
        headers.insert("Referer", self.base_url.parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        let request = self.request.get(format!("{}/Mobile/Indexs/next/type/2", self.base_url))
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
            // 有可能是开奖时间0于服务器时间和本地时间，说明开奖时间有问题
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
            // // println!("下次开奖时间是: {open_time_s}");
            // // println!("开奖时间戳: {open_time}");
            // // println!("服务器时间: {server_time}");
            // // println!("本地器时间: {since_epoch}");
            // // println!("开奖剩余时间: {}秒", duration.as_secs());
            return Ok((duration, section));
        } else {
            // println!("get json data error:{body}");
            return Err(Box::from("may be need use proxy！"));
        }
    }

    // 获取data
    pub async fn get_result(&self) -> Result<(), Box<dyn Error>> {
        // println!("get data......");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json, text/javascript, */*; q=0.01".parse()?);
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8".parse()?);
        headers.insert("Proxy-Connection", "keep-alive".parse()?);
        headers.insert("Referer", self.base_url.parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        let request = self.request.get(format!("{}/Mobile/Indexs/myOpens/type/2/pz/{}/page/1", self.base_url, self.page_size))
            .headers(headers);
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        // 将文本字符串转为结构体: 会被防火墙截胡,导致返回的防诈骗广告
        if body.contains("total") {
            let map_value: Value = serde_json::from_str(&body).unwrap();
            // 需要处理四把和五把逻辑：
            self.handle_res_four(&map_value).await?;
            self.handle_res_five(&map_value).await?;
            Ok(())
        } else {
            // println!("data error:{body}");
            Err(Box::from("may need open proxy！".to_string()))
        }
    }

    // 获取盲猜
    pub async fn get_calculate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // println!("get feature data......");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json, text/javascript, */*; q=0.01".parse()?);
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8".parse()?);
        headers.insert("Proxy-Connection", "keep-alive".parse()?);
        headers.insert("Referer", self.base_url.parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        let request = self.request.get(format!("{}/Mobile/Indexs/yuce/type/2/pz/{}/1", self.base_url, self.page_size))
            .headers(headers);
        let response = request.send().await?;
        let body = response.text().await?;
        // 将文本字符串转为结构体
        if body.contains("total") {
            let map_value: Value = serde_json::from_str(&body).unwrap();
            // 处理最新4条和5条数据
            self.handle_cal_four(&map_value).await?;
            self.handle_cal_five(&map_value).await?;
            Ok(())
        } else {
            // println!("get json eroor:{body}");
            Err(Box::from("may need open proxy！".to_string()))
        }
    }

    // 处理4结果数据是否发送微信通知
    pub async fn handle_res_four(&self, result: &Value) -> Result<(), Box<dyn std::error::Error>> {
        let res_data = result.get("data").unwrap().as_array().unwrap();
        // // println!("处理data: {res_data:?}");
        let last_four = &res_data[0..4];
        // // println!("获取最新的四条结果数据是: {last_four:?}");
        let mut open_nums: Vec<&str> = vec![];
        let mut single_dou: Vec<&str> = vec![];
        for data in last_four.iter() {
            let num = data.get("openNum").unwrap().as_i64().unwrap();
            // 判断10
            if num <= 13 {
                open_nums.push("0");
            } else {
                open_nums.push("1");
            }
            // 判断911
            if num % 2 == 0 {
                // println!("double：{num:?}");
                single_dou.push("11");
            } else {
                // println!("single：{num:?}");
                single_dou.push("9");
            }
        }
        // // println!("open data: {open_nums:?} 911结果是：{single_dou:?}");
        // 判断10
        if open_nums == vec!["1", "1", "1", "1"] {
            let send_msg = String::from("1111");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("数据: {}", send_msg))).await?;
        } else if open_nums == vec!["0", "0", "0", "0"] {
            let send_msg = String::from("0000");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("数据: {}", send_msg))).await?;
        } else {
            // println!("不用关心的10结果");
        }
        // 判断911
        if single_dou == vec!["9", "9", "9", "9"] {
            let send_msg = String::from("9999");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("数据: {}", send_msg))).await?;
        } else if single_dou == vec!["11", "11", "11", "11"] {
            let send_msg = String::from("11111111");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("数据: {}", send_msg))).await?;
        } else {
            // println!("不用关心的911结果");
        }
        Ok(())
    }


    // 处理5条数据是否发送微信通知
    pub async fn handle_res_five(&self, result: &Value) -> Result<(), Box<dyn std::error::Error>> {
        let res_data = result.get("data").unwrap().as_array().unwrap();
        // // println!("处理data: {res_data:?}");
        let last_four = &res_data[0..5];
        // // println!("获取最新的四条结果数据是: {last_four:?}");
        let mut open_nums: Vec<&str> = vec![];
        let mut single_dou: Vec<&str> = vec![];
        for data in last_four.iter() {
            let num = data.get("openNum").unwrap().as_i64().unwrap();
            // 判断10
            if num <= 13 {
                open_nums.push("0");
            } else {
                open_nums.push("1");
            }
            // 判断911
            if num % 2 == 0 {
                // println!("11数：{num:?}");
                single_dou.push("11");
            } else {
                // println!("9数：{num:?}");
                single_dou.push("9");
            }
        }
        // println!("开奖的五个结果10是: {open_nums:?} 911结果是：{single_dou:?}");
        // 判断10
        if open_nums == vec!["0", "1", "0", "1", "0"] {
            let send_msg = String::from("01010");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("数据: {}", send_msg))).await?;
        } else if open_nums == vec!["1", "0", "1", "0", "1"] {
            let send_msg = String::from("10101");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("数据: {}", send_msg))).await?;
        } else {
            // println!("不用关心的10结果");
        }
        // 判断911
        if single_dou == vec!["9", "11", "9", "11", "9"] {
            let send_msg = String::from("9119119");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("数据: {}", send_msg))).await?;
        } else if single_dou == vec!["11", "9", "11", "9", "11"] {
            let send_msg = String::from("11911911");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("数据: {}", send_msg))).await?;
        } else {
            // println!("不用关心的911结果");
        }
        Ok(())
    }


    // 处理4盲猜是否发送微信通知
    pub async fn handle_cal_four(&self, result: &Value) -> Result<(), Box<dyn std::error::Error>> {
        let res_data = result.get("data").unwrap().as_array().unwrap();
        let four_data = &res_data[1..5];
        // // println!("获取最新的四条预测数据是: {four_data:?}");
        let mut left_color: Vec<&str> = vec![];
        let mut right_color: Vec<&str> = vec![];
        for data in four_data.iter() {
            let r1_r = data.get("r1_r").unwrap().as_i64().unwrap();
            let r2_r = data.get("r2_r").unwrap().as_i64().unwrap();
            // 判断左边颜色：0b 1r
            if r1_r == 0 {
                left_color.push("b");
            } else {
                left_color.push("r");
            }
            // 判断右边颜色：0b 1r
            if r2_r == 0 {
                right_color.push("b");
            } else {
                right_color.push("r");
            }
        }
        // println!("预测的四个结果是: {left_color:?} r: {right_color:?}");
        // 判断左边颜色
        if left_color == vec!["b", "b", "b", "b"] {
            let send_msg = String::from("lbbbb");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("盲猜: {}", send_msg))).await?;
        } else if left_color == vec!["r", "r", "r", "r"] {
            let send_msg = String::from("lrrrr");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("盲猜: {}", send_msg))).await?;
        } else {
            // println!("l不用关心的结果");
        }
        // 判断右边颜色
        if right_color == vec!["b", "b", "b", "b"] {
            let send_msg = String::from("rbbbb");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("盲猜: {}", send_msg))).await?;
        } else if right_color == vec!["r", "r", "r", "r"] {
            let send_msg = String::from("rrrrr");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("盲猜: {}", send_msg))).await?;
        } else {
            // println!("r不用关心的结果");
        }
        Ok(())
    }

    // 处理5条结果是否发送微信通知
    pub async fn handle_cal_five(&self, result: &Value) -> Result<(), Box<dyn std::error::Error>> {
        let res_data = result.get("data").unwrap().as_array().unwrap();
        let four_data = &res_data[1..6];
        // // println!("获取最新的四条预测数据是: {four_data:?}");
        let mut left_color: Vec<&str> = vec![];
        let mut right_color: Vec<&str> = vec![];
        for data in four_data.iter() {
            let r1_r = data.get("r1_r").unwrap().as_i64().unwrap();
            let r2_r = data.get("r2_r").unwrap().as_i64().unwrap();
            // 判断左边颜色：0b 1r
            if r1_r == 0 {
                left_color.push("b");
            } else {
                left_color.push("r");
            }
            // 判断右边颜色：0b 1r
            if r2_r == 0 {
                right_color.push("b");
            } else {
                right_color.push("r");
            }
        }
        // println!("预测的五个结果是: {left_color:?} r: {right_color:?}");
        // 判断左边颜色
        if left_color == vec!["b", "r", "b", "r", "b"] {
            let send_msg = String::from("lbrbrb");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("盲猜: {}", send_msg))).await?;
        } else if left_color == vec!["r", "b", "r", "b", "r"] {
            let send_msg = String::from("lrbrbr");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("盲猜: {}", send_msg))).await?;
        } else {
            // println!("l不用关心的结果");
        }
        // 判断右边颜色
        if right_color == vec!["b", "r", "b", "r", "b"] {
            let send_msg = String::from("rbrbrb");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("盲猜: {}", send_msg))).await?;
        } else if right_color == vec!["r", "b", "r", "b", "r"] {
            let send_msg = String::from("rrbrbr");
            // println!("{}", send_msg);
            self.wx_pusher.push_summary(&send_msg, String::from(format!("盲猜: {}", send_msg))).await?;
        } else {
            // println!("r不用关心的结果");
        }
        Ok(())
    }
}