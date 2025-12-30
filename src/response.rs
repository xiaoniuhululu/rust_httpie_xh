/// response.rs
use colored::Colorize;
use mime::Mime;
use reqwest::Response;

pub async fn render(resp: Response) -> anyhow::Result<()> {
    //打印状态行：协议版本 + 状态码
    println!(
        "{} {}\n",
        format!("{:?}", resp.version()).blue(),
        resp.status().to_string().blue()
    );
    //打印 Header，为了便于区分，把 Key 染成绿色
    for (k, v) in resp.headers() {
        println!("{}: {:?}", k.to_string().green(), v);
    }
    // Header 和 Body 之间空一行，符合 HTTP 协议视觉习惯
    println!();

    //处理 Body
    let mime = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<Mime>().ok());

    // 获取原始 Body 文本
    let body = resp.text().await?;

    // 如果是 JSON，则进行格式化输出
    if let Some(m) = mime {
        if m.type_() == mime::APPLICATION && m.subtype() == mime::JSON {
            if let Ok(pretty) = jsonxf::pretty_print(&body) {
                println!("{}", pretty.cyan());
                return Ok(());
            }
        }
    }

    // 如果不是 JSON 或者格式化失败，就原样打印
    println!("{}", body);
    
    Ok(())
}
