#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    let client = HttpClient::new();
    // 事前にホスト側（Mac）でリポジトリルートから `python3 -m http.server 8001` を起動しておくこと
    // ポート8000はtaskdog-serverがIPv4の127.0.0.1:8000を占有していて、
    // QEMUゲストからのhost.test:8000宛の通信がそちらに届いてしまうため8001を使う
    match client.get("host.test".to_string(), 8001, "test.html".to_string()) {
        Ok(res) => {
            print!("response:\n{:#?}", res);
        }
        Err(e) => {
            print!("error:\n{:#?}", e);
        }
    }
    0
}

entry_point!(main);