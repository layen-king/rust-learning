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
    method: Method,
    url: String,
    params: HashMap<String, String>,
}

impl Request {
    fn new(method: Method, url: String, params: HashMap<String, String>) -> Request {
        Request {
            method,
            url,
            params,
        }
    }
}

/// 生成一个枚举
fn make_method(str: &str) -> Method {
    match str {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "OPTIONS" => Method::OPTIONS,
        _ => Method::NULL,
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
    let req = v[0].split_whitespace().collect::<Vec<&str>>();
    let method = make_method(req[0]);
    let req = Request::new(method, req[1].to_string(), params);
    req
}

/// 处理tcp流
pub fn handle_connect(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 1024];
    stream.read(&mut buf)?;
    let request = String::from_utf8_lossy(&buf);
    let r = parse_url(&request);
    println!("{:?}", r);
    match r.method {
        Method::GET => {
            read_file(r.url);
        }
        Method::POST => {}
        Method::PUT => {}
        Method::OPTIONS => {}
        Method::DELETE => {}
        _ => {}
    }
    Ok(())
}

/// ## 读取文件
/// #### [file_path]: 文件路径
/// #### [result]: 返回文件字符串,已经处理错误
fn read_file(file_path: String) -> String {
    
    todo!("")
}
