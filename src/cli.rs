/// cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "httprs_cli",version = "1.0.0", author="xiaoniuhululu.com", about = "httpie-rs mini", long_about = None)] // 设置名称、版本、作者、简介等元数据
pub struct Cli { // 定义主 CLI 结构体，自动生成 help 信息
    #[command(subcommand)]//设置子命令
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {//定义支持的子命令列表
    Get(RequestArgs),
    Post(RequestArgs),
    Put(RequestArgs),
    Delete(RequestArgs),
}

#[derive(Parser)]
pub struct RequestArgs {
    pub url: String, // 位置参数：URL
    pub items: Vec<String>, // 位置参数：后续所有的 kv 对，收集为一个字符串数组
}
