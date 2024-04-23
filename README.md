[![qcos](https://github.com/928799934/rust-qcos/actions/workflows/qcos.yml/badge.svg?branch=master)](https://github.com/928799934/rust-qcos/actions/workflows/qcos.yml)

**异步版本** `async`/`await`

本包提供腾讯云对象存储(cos) 基本的操作，包括`bucket`创建及删除，对象的上传(支持分块传输)、下载、删除等。后续有时间会补充其他接口的实现。

# How to use

```rust
use qcos::acl::{AclHeader, ObjectAcl};
use qcos::client::Client;
use qcos::objects::{mime, ErrNo, Objects};

#[tokio::main]
async fn main() {
    let client = Client::new(
        "Your secrect id",
        "Your secrect key",
        "bucket name",
        "region",
    );
    let mut acl_header = AclHeader::new();
    acl_header.insert_object_x_cos_acl(ObjectAcl::PublicRead);
    let data = std::fs::read("test.png").unwrap();
    let res = client.put_object(mime::IMAGE_PNG, "test.png", data, Some(&acl_header)).await;
    if res.error_no == ErrNo::SUCCESS {
        println!("success");
    } else {
        println!("{}", res.error_message);
    }
}

```

如果操作成功，会打印出`success`, 否则会打印出失败原因。

# Installation

insert into your project's cargo.toml block next line

```
[dependencies]
qcos = "0.0.1"
```
