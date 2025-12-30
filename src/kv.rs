/// kv.rs
use anyhow::{anyhow, Result};
use std::str::FromStr;

/// 定义核心数据模型,kv键值对
#[derive(Debug, Clone)]
pub enum Kv {
    Header(String, String), // 对应 header:value
    Query(String, String),  // 对应 query==value
    Json(String, serde_json::Value), // 对应 json=value 或 json:=value
}
/// 实现 FromStr trait
/// 解析键值对字符串，将其转换为强类型的 Kv 枚举
impl FromStr for Kv {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 优先级很重要：先判断复杂的，再判断简单的

        if let Some((k, v)) = s.split_once(":=") {// 处理 JSON 原生类型 (key:=value)
            Ok(Kv::Json(k.into(), serde_json::from_str(v)?))
        } else if let Some((k, v)) = s.split_once("==") { // 处理 Query 参数 (key==value)
            Ok(Kv::Query(k.into(), v.into()))
        } else if let Some((k, v)) = s.split_once(':') { // 处理 Header (key:value)
            Ok(Kv::Header(k.into(), v.into()))
        } else if let Some((k, v)) = s.split_once('=') { // 处理普通 JSON 字符串 (key=value)
            Ok(Kv::Json(k.into(), serde_json::Value::String(v.into())))
        } else {
            Err(anyhow!("invalid argument: {}, expected format like key=value", s))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header() {
        let kv = "Authorization:token".parse().unwrap();
        matches!(kv, Kv::Header(_, _));
    }

    #[test]
    fn parse_query() {
        let kv: Kv = "page==1".parse().unwrap();
        assert!(matches!(kv, Kv::Query(k, v) if k == "page" && v == "1"));
    }

    #[test]
    fn parse_json_string() {
        let kv: Kv = "name=jack".parse().unwrap();
        assert!(matches!(kv, Kv::Json(k, v) if k == "name" && v == "jack"));
    }

    #[test]
    fn parse_json_raw() {
        let kv: Kv = "age:=18".parse().unwrap();
        // 检查 json 的数字类型
        assert!(matches!(kv, Kv::Json(k, v) if k == "age" && v == 18));
    }

    #[test]
    fn parse_json_obj() {
        // 测试复杂 JSON 对象
        let kv: Kv = r#"user:={"name":"rose"}"#.parse().unwrap();
        if let Kv::Json(k, v) = kv {
            assert_eq!(k, "user");
            assert_eq!(v["name"], "rose");
        } else {
            panic!("expected json obj");
        }
    }

    #[test]
    fn invalid_kv() {
        let result = "xxx".parse::<Kv>();
        assert!(result.is_err());
    }
}