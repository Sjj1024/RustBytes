mod toutiao;
mod douyin;
mod utils;

#[tokio::main]
async fn main() {
    // run lottery main
    douyin::live::lottery_main().await;
}
