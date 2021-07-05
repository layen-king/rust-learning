use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
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
    let req = v
        .get(0)
        .unwrap_or(&"")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let method = make_method(req.get(0).unwrap_or(&""));
    let url = req.get(1).unwrap_or(&"");
    let req = Request::new(method, url.to_string(), params);
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
            let response = make_response(&r.url);
            stream.write(response.as_bytes())?;
            stream.flush()?;
        }
        Method::POST => {}
        Method::PUT => {}
        Method::OPTIONS => {}
        Method::DELETE => {}
        _ => {}
    }
    Ok(())
}

#[allow(dead_code)]
/// ## 读取文件
/// #### [file_path]: 文件路径
/// #### [result]: 返回文件字符串,已经处理错误
fn read_file(file_path: &str) -> Result<String> {
    let mut full_path = String::from("static");
    let mut path = file_path.to_owned();
    if path == "/index" || path == "/" {
        path = String::from("/index.html")
    }
    full_path = full_path + &path;
    println!("path: {:?}", full_path);
    let mut f = File::open(full_path)?;
    let mut file_content = String::new();
    f.read_to_string(&mut file_content)?;
    Ok(file_content)
}

/// 构造返回文件
fn make_response(file_path: &str) -> String {
    let file_content = read_file(file_path);
    println!("mime_types : {:?}", mime_types);
    if let Ok(file_content) = file_content {
        format!(
            "HTTP/1.1 200 Ok\r\nContent-Length: {}\r\nContent-type: text/html; charset=utf-8\r\n\r\n{}",
            file_content.len(),
            file_content
        )
    } else {
        String::from("HTTP/1.1 404 File not found")
    }
}
