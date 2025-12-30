# ⚡ rust-httpie-xh (HTTPie-rs)

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

> **A blazing fast, lightweight HTTP client for the command line.**  
> Think `curl`, but human-readable and powered by Rust. 🦀

`httprs` is a modern CLI HTTP client designed to make API testing painless. It mimics the intuitive syntax of [HTTPie](https://httpie.io/) but leverages the performance and safety of the Rust ecosystem.



## ✨ Features

- **Intuitive Syntax**: Forget `-H "Content-Type: application/json"`. Just type `key=value`.
- **JSON by Default**: Built-in JSON support with syntax highlighting.
- **Blazing Fast**: Written in Rust, using `tokio` for async I/O.
- **Smart Formatting**: Automatic syntax coloring for headers and JSON bodies.
- **Type-Safe Parsing**: 
  - `key:value` for Headers
  - `key=value` for JSON Strings
  - `key:=value` for JSON Raw data (Booleans, Numbers, Arrays)
  - `key==value` for Query Parameters
- Currently supports only GET, POST, PUT, and DELETE requests.

## 📦 Installation

### From Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

```bash
git clone https://github.com/xiaoniuhululu/rust_httpie_xh.git
cd rust_httpie_xh

#========GET=========
# get base
cargo run get https://httpbin.org/get

# 带查询参数 (Result: ?a=1&b=2)
cargo run get https://httpbin.org/get a==1 name==xiaoniuhululu

# 带请求头 (Result: Headers 中包含 X-Api-Key 和 User-Agent)
cargo run get https://httpbin.org/get X-Api-Key:abc-123 User-Agent:HttpRs/1.0

# 混合：参数 + 请求头
cargo run get https://httpbin.org/get search==rust lang==cn Authorization:BearerToken

#========POST=========
# 纯字符串字段 (Result: {"greeting": "wx", "name": "xiaoniuhululu"})
cargo run post https://httpbin.org/post greeting=wx name=xiaoniuhululu

# 混合类型：字符串 + 数字 + 布尔值 (注意 := 的用法)
cargo run post https://httpbin.org/post name=xiaoniuhululu id:=100 is_admin:=true

# 测试覆盖：带浮点数和 null
cargo run post https://httpbin.org/post score:=99.5 parent:=null

#========PUT=========
# PUT
cargo run put https://httpbin.org/put name=jack age:=18

#========DELETE=========
# 常见用法：通过 URL Query 删除
cargo run delete https://httpbin.org/delete id==1

# 特殊用法：部分 API 要求在 DELETE Body 中带数据
cargo run delete https://httpbin.org/delete ids:='[10, 20]'