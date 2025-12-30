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

## 📦 Installation

### From Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

```bash
git clone https://github.com/yourusername/rust_httpie_xh.git
cd rust_httpie_xh
cargo install --path .