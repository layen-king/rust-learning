use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use std::{fs::File, io::Read};

/// 配置项结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// 配置的ip,127.0.0.1
    pub ip: String,
    /// 监听的端口号,3303
    pub port: String,
    /// 主页文件
    pub root_file: String,
    /// 主页路由
    pub index: Vec<String>,
    /// 根路径
    pub root_dir: String,
    /// 允许的访问静态文件夹
    pub allow_folders: Vec<String>,
    /// 允许的静态文件类型
    pub allow_file_types: Vec<String>,
    /// 允许访问api
    pub allow_apis: Vec<String>,
}
/// 读取配置文件
pub fn read_config() -> Result<Config, std::io::Error> {
    let mut file_str = String::new();
    File::open("./config.json")?.read_to_string(&mut file_str)?;
    let config: Config = serde_json::from_str(&file_str)?;
    println!("config: {:?}", config);
    Ok(config)
}

/// 解析tcp请求,获取请求地址
fn parse_url(request: &str) -> Vec<&str> {
    let mut v: Vec<&str> = request.split(' ').collect();
    // println!("request: {:?}", v);
    if v.len() < 2 {
        for _ in v.len()..2 {
            v.push("")
        }
    }
    v
}

/// 处理tcp连接
pub fn connect_handler(mut stream: TcpStream, config: Config) -> Result<(), std::io::Error> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    let req_string = String::from_utf8_lossy(&buffer[..]);
    let req_type = parse_url(&req_string)[0];
    let mut req_path = parse_url(&req_string)[1];
    println!("request url: {},type: {:?}", req_path, req_type);
    match req_type {
        "GET" => {
            // 若在请求地址在配置项index字段内,直接读取root_file文件
            for url in config.index.iter() {
                if url == req_path {
                    req_path = &config.root_file;
                    break;
                }
            }
            if is_allowed(req_path, &config.allow_folders) {
                let content = read_file(config.root_dir, req_path);
                println!("content is {}", content);
                stream.write(content.as_bytes())?;
                stream.flush()?;
            } else {
                if is_allowed(req_path, &config.allow_apis) {
                    // todo 调用api
                    println!("transfer api")
                } else {
                    let content = String::from("禁止访问此资源");
                    // 不允许的文件类型
                    let response = format!(
                        "HTTP/1.1 200 Ok\r\nContent-Length: {}\r\nContent-type: text/html; charset=utf-8\r\n\r\n{}",
                        content.len(),
                        content
                    );
                    stream.write(response.as_bytes())?;
                    stream.flush()?;
                }
            }
        }
        "POST" => {
            // 执行控制器函数
        }
        "DELETE" => {}
        "PUT" => {}
        _ => {}
    }

    Ok(())
}

/// 是否是允许路径
fn is_allowed(req_path: &str, allow_array: &Vec<String>) -> bool {
    if allow_array.is_empty() {
        return true;
    }
    for allow in allow_array.iter() {
        if req_path.starts_with(allow) {
            return true;
        }
    }
    false
}

/// 是否是允许的静态文件类型
#[allow(dead_code)]
fn is_allowed_file_type(file_path: &str, allow_file_types: &Vec<String>) -> Result<bool, ()> {
    // 若不填写允许静态文件类型,允许所有文件类型
    if allow_file_types.is_empty() {
        return Ok(true);
    }
    let file_type = Path::new(file_path)
        .file_name()
        .ok_or_else(|| {})?
        .to_str()
        .ok_or_else(|| {})?;

    for allow_file_type in allow_file_types.iter() {
        if allow_file_type == file_type {
            return Ok(true);
        }
    }

    Ok(false)
}

/// 读取文件,返回文件串.若文件不存在,返回String
fn read_file(root_dir: String, path: &str) -> String {
    println!("read file {}", path);
    if let Ok(mut file) = File::open(format!("{}{}", root_dir, path)) {
        let mut file_content = String::new();
        // todo 若文件过大,使用文件流发送
        file.read_to_string(&mut file_content).ok();
        // todo 根据请求类型,生成内容
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            file_content.len(),
            file_content
        )
    } else {
        let file_content = String::from("404 NotFound");
        format!(
            "HTTP/1.1 404 NotFound\r\nContent-Length: {}\r\n\r\n{}",
            file_content.len(),
            file_content
        )
    }
}
