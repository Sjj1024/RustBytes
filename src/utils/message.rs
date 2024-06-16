use reqwest::Client;

// 发送消息的实现
pub struct WxPusher {
    request: Client,
    app_token: String,
    base_url: String,
    user_ids: Vec<String>,
}


impl WxPusher {
    pub fn new(token: String) -> Self {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        // 推送给的客户UUID
        // UID_jhavxjntRoEkhw6xGVqSEZGNFDbD woziji
        // 下面这是客户的UUID UID_t4vpwPTSyoZ2jcVoozq18hreLOP3客户
        let users = vec![String::from("UID_jhavxjntRoEkhw6xGVqSEZGNFDbD"), String::from("UID_t4vpwPTSyoZ2jcVoozq18hreLOP3")];
        // let users = String::from("UID_t4vpwPTSyoZ2jcVoozq18hreLOP3");
        return WxPusher { request: client, app_token: token, base_url: String::from("https://wxpusher.zjiecode.com/api/send/message"), user_ids: users };
    }

    pub async fn test_push(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("测试发送微信消息");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        let json = serde_json::json!({
            "appToken": self.app_token,
            "content": "<h1>H1标题</h1>",
            "summary": "消息摘要2",
            "contentType": 2,
            "verifyPayType": 0,
            "uids": self.user_ids
        });
        let request = self.request.request(reqwest::Method::POST, &self.base_url)
            .headers(headers)
            .json(&json);
        let response = request.send().await?;
        let body = response.text().await?;
        println!("send wx: {}", body);
        Ok(())
    }


    pub async fn push_msg(&self, msg: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("测试发送微信消息");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        let json = serde_json::json!({
            "appToken": self.app_token,
            "content": msg,
            "summary": "消息摘要2",
            "contentType": 2,
            "verifyPayType": 0,
            "uids": self.user_ids
        });
        let request = self.request.request(reqwest::Method::POST, &self.base_url)
            .headers(headers)
            .json(&json);
        let response = request.send().await?;
        let body = response.text().await?;
        println!("send wx: {}", body);
        Ok(())
    }


    pub async fn push_summary(&self, summary: &String, msg: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("测试发送微信消息");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        let json = serde_json::json!({
            "appToken": self.app_token,
            "content": msg,
            "summary": summary,
            "contentType": 2,
            "verifyPayType": 0,
            "uids": self.user_ids
        });
        let request = self.request.request(reqwest::Method::POST, &self.base_url)
            .headers(headers)
            .json(&json);
        let response = request.send().await?;
        let body = response.text().await?;
        println!("send wx: {}", body);
        Ok(())
    }
}