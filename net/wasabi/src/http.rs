extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use alloc::format;
use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;
use alloc::vec::Vec;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host:String, port: u16, path: String)-> Result<HttpResponse, Error> {
        // ホスト名からIPアドレスを返す
        // ホスト名からIPアドレスを返すことを正引きという
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP addresses {:#?}", e
                )))
            }
        };

        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
        }

        // ソケットアドレスの定義
        let socket_addr: SocketAddr = (ips[0], port).into();

        // ストリームの構築
        // はじめに送信側と受信側でコネクションを確立する
        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network("Failed to connect to TCP stream".to_string()))
            }
        };

        // リクエストラインの構築
        // メソット名とパス名とHTTPバージョンをホワイトスペースで繋げる
        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\r\n");

        // ヘッダの追加
        request.push_str("Host: ");
        request.push_str(&host);
        request.push_str("\r\n");
        request.push_str("Accept: text/html\r\n");
        request.push_str("Connection: close\r\n");
        request.push_str("\r\n");

        // リクエストの送信
        // rustでは使わない変数については先頭に_をつけて定義することで、それを意味させることができる
        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network("Failed to send a request to TCP strean".to_string()))
            }
        };

        // レスポンスの受信
        let mut received = Vec::new();
        loop {
            let mut buf = [0u8; 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::Network("Failed to receive a request from TCP stream".to_string()))
                }
            };
            if bytes_read == 0 {
                break;
            }
            received.extend_from_slice(&buf[..bytes_read]);
        }

        // HTTPレスポンスの構築
        match core::str::from_utf8(&received) {
            Ok(response) => HttpResponse::new(response.to_string()),
            Err(e) => Err(Error::Network(format!("invalid received response {}", e)))
        }
    }
}