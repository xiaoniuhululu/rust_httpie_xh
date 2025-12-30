/// main.rs
mod cli;
mod kv;
mod request;
mod response;

use clap::Parser;
use reqwest::{Client, Method};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    // 如果参数格式不对，clap 会在这里直接打印 help 信息并退出，不用我们操心
    let cli = cli::Cli::parse();
    // 初始化 HTTP 客户端
    let client = Client::new();

    // 匹配子命令，确定 HTTP 方法和参数
    let (method, args) = match cli.cmd {
        cli::Command::Get(a) => (Method::GET, a),
        cli::Command::Post(a) => (Method::POST, a),
        cli::Command::Put(a) => (Method::PUT, a),
        cli::Command::Delete(a) => (Method::DELETE, a),
    };

    // 解析 kv 参数
    let items = args
        .items
        .iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;
    // 构建请求
    let req = request::build_request(&client, method, &args.url, items)?;
    // 发送请求
    let resp = req.send().await?;
    // 渲染响应
    response::render(resp).await
}
