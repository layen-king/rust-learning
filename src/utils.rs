use std::collections::HashMap;
// use anyhow;
use std::io::{Read, Result};
use std::net::TcpStream;

#[derive(Debug)]
#[allow(dead_code)]
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    NULL,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Request {
    method: String,
    url: String,
    params: HashMap<String, String>,
}

impl Request {
    fn new(method: String, url: String, params: HashMap<String, String>) -> Request {
        Request {
            method,
            url,
            params,
        }
    }
}

/// 解析url成一个结构体
fn parse_url(requst: &str) -> Request {
    let v: Vec<&str> = requst.split("\r\n").collect();
    let mut params: HashMap<String, String> = HashMap::new();
    for str in v.iter() {
        if str.contains(":") {
            let str = *str;
            let kv = str.split(":").collect::<Vec<&str>>();
            params.insert(kv[0].to_owned(), kv[1].to_owned());
        }
    }
    println!("v[0] {:?}", v[0]);
    let req = v[0].split_whitespace().collect::<Vec<&str>>();
    let req = Request::new(req[0].to_string(), req[1].to_string(), params);
    req
}

/// 处理tcp流
pub fn handle_connect(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 1024];
    stream.read(&mut buf)?;
    let request = String::from_utf8_lossy(&buf);
    let r = parse_url(&request);
    println!("{:?}", r);
    Ok(())
}
