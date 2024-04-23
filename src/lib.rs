/*!
本包提供腾讯云对象存储(cos)基本接口封装.

基本用法:

```
 use rust_qcos::client::Client;
 use rust_qcos::objects::Objects;
 use mime;

 #[tokio::main]
 async fn main() {
    let client = Client::new("foo", "bar", "qcloudtest-1256650966", "ap-guangzhou");
    /// 上传文件
    let data = std::fs::read("Cargo.toml").unwrap();
    let res = client.put_object(mime::TEXT_PLAIN_UTF_8, "Cargo.toml", data, None).await;
    /// 删除文件
    let res = client.delete_object("Cargo.toml").await;
 }
```
*/

pub mod acl;
pub mod bucket;
pub mod client;
pub mod objects;
pub mod request;
pub mod service;
pub mod signer;
