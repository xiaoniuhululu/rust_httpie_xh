/// request.rs
use reqwest::{Client, RequestBuilder};
use serde_json::{Map, Value};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue}; // 1. 显式引入 header 相关类型

use crate::kv::Kv;

pub fn build_request(
    client: &Client,
    method: reqwest::Method,
    url: &str,
    items: Vec<Kv>,
) -> anyhow::Result<RequestBuilder> {
    let mut headers = HeaderMap::new();
    let mut query = Vec::new();
    let mut json = Map::new();

    for item in items {
        match item {
            Kv::Header(k, v) => {
                //这里需要显式指定解析的目标类型HeaderName，HeaderValue；因为rust编译器的类型推导能力有限
                headers.insert(k.parse::<HeaderName>()?, v.parse::<HeaderValue>()?);
            }
            Kv::Query(k, v) => query.push((k, v)),
            Kv::Json(k, v) => {
                json.insert(k, v);
            }
        }
    }

    // 组装 RequestBuilder
    let mut req = client.request(method, url).headers(headers);

    // 只有当 query 不为空时才挂载
    if !query.is_empty() {
        req = req.query(&query);
    }

    // 如果有 JSON 字段，序列化为 Body
    if !json.is_empty() {
        req = req.json(&Value::Object(json));
    }

    Ok(req)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{Client, Method};

    #[test]
    fn build_post_request() {
        let client = Client::new();
        let items = vec![
            Kv::Json("name".into(), "jack".into()),
            Kv::Header("X-Test".into(), "1".into()),
        ];

        let req = build_request(&client, Method::POST, "http://example.com", items).unwrap();
        let req = req.build().unwrap();

        assert_eq!(req.method(), Method::POST);
        assert!(req.headers().contains_key("x-test"));
    }

    #[test]
    fn build_delete_with_body() {
        let client = Client::new();
        let items = vec![Kv::Json("id".into(), 1.into())];

        let req = build_request(&client, Method::DELETE, "http://example.com", items).unwrap();
        let req = req.build().unwrap();

        assert_eq!(req.method(), Method::DELETE);
    }
}

