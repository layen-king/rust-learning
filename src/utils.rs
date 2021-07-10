use anyhow::{Context, Result};
use mime_types;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
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
    is_file: bool,
    file_type: String,
    params: HashMap<String, String>,
}

impl Request {
    fn new(
        method: Method,
        url: String,
        is_file: bool,
        file_type: String,
        params: HashMap<String, String>,
    ) -> Request {
        Request {
            method,
            url,
            is_file,
            file_type,
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
    let mut url = *req.get(1).unwrap_or(&"/index.html");
    if url == "/index" || url == "/" {
        url = "/index.html"
    }
    let is_file = is_file(url);
    let mut file_type = String::from("");
    if is_file {
        file_type = parse_file_type(url);
    }
    let req = Request::new(method, url.to_owned(), is_file, file_type, params);
    req
}

/// ## 判断是否为文件
fn is_file(url: &str) -> bool {
    let re = Regex::new(r"^.+?\.[\w\d]+$").unwrap();
    re.is_match(url)
}

/// ## 解析文件类型
fn parse_file_type(url: &str) -> String {
    let re = Regex::new(r"^.+?\.([\w\d]+)$").unwrap();
    let caps = re.captures(url).unwrap();
    println!("caps {:?}", caps);
    caps[1].to_owned()
}

/// ## 处理tcp流
pub fn handle_connect(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 1024];
    stream.read(&mut buf)?;
    let request = String::from_utf8_lossy(&buf);
    let r = parse_url(&request);
    match r.method {
        Method::GET => {
            // todo 判断是静态文件还是接口请求
            // todo 静态文件 -> 解析文件类型,判断是否支持,不支持直接返回错误
            // todo 读取文件,不存在返回404
            // ----
            // todo 接口请求 -> 查询路由列表,不存在返回404
            // todo 若存在路由 -> 执行路由函数然后返回
            let response = make_response(&r)?;
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

/// ## 读取文件
/// #### [file_path]: 文件路径
/// #### [result]: 返回文件字符串,已经处理错误
fn read_file(file_path: &str) -> Result<String> {
    let mut full_path = String::from("static");
    full_path = full_path + file_path;
    let f =
        File::open(full_path.to_owned()).context(format!("can not find file: {}", &full_path))?;
    let mut buf_reader = BufReader::new(f);
    let mut file_content = String::new();
    buf_reader.read_to_string(&mut file_content)?;
    Ok(file_content)
}

/// ## 构造返回文件
/// ## [request] 请求解析结构体
fn make_response(request: &Request) -> Result<String> {
    let mime_type = mime_types::get_mime_type(&request.file_type)?;
    let file_content = read_file(&request.url)?;
    let res = format!(
        "HTTP/1.1 200 Ok\r\nContent-Length: {}\r\nContent-type: {}; charset=utf-8\r\n\r\n{}",
        file_content.len(),
        mime_type,
        file_content
    );
    Ok(res)
}
