use serde::{Deserialize, Serialize};
//use serde_json::Result;
use bufstream::BufStream;
use std::path::Path;
use std::{fs::File, io::Read};
use std::{io::BufRead, net::TcpStream};

/// 配置项结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// 配置的ip,127.0.0.1
    pub ip: String,
    /// 监听的端口号,3303
    pub port: String,
    /// 主页文件
    pub root_dir: String,
    /// 主页路由
    pub index: Vec<String>,
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
    let v: Vec<&str> = request.split(' ').collect();
    println!("request: {:?}", v);
    v
}

/// 处理tcp连接
pub fn connect_handler(stream: TcpStream, config: Config) {
    let mut req_string = String::new();
    let mut buf = BufStream::new(stream);
    // todo 读取流可能触发IO错误
    buf.read_line(&mut req_string).unwrap();
    let req_type = parse_url(&req_string)[0];
    let mut request_path = parse_url(&req_string)[1];
    println!("request url: {},config: {:?}", request_path, config);
    match req_type {
        "GET" => {
            // 若在请求地址在配置项index字段内,直接读取root_dir文件
            for url in config.index.iter() {
                if url == request_path {
                    request_path = &config.root_dir;
                    break;
                }
            }
            if is_allowed(request_path, &config.allow_folders) {
                if is_allowed_file_type(request_path, &config.allow_file_types) {
                    read_file(request_path)
                }else{
                    // todo 不允许的文件类型
                }
            } else {
                if is_allowed(request_path, &config.allow_apis) {
                    // todo 调用api
                    println!("transfer api")
                }else{
                    // todo 返回404
                    println!("404")
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
}

/// 是否允许
fn is_allowed(request_path: &str, allow_array: &Vec<String>) -> bool {
    for allow in allow_array.iter() {
        if request_path.starts_with(allow) {
            return true;
        }
    }
    false
}

/// 是否是允许的静态文件类型
fn is_allowed_file_type(file_path: &str, allow_file_types: &Vec<String>) -> bool {
    // 若不填写允许静态文件类型,允许所有文件类型
    if allow_file_types.len() == 0 {
        return true;
    }
    let _path = Path::new(file_path);
    let file_name = _path.file_name();

    if let Some(file_name) = file_name {
        if let Some(file_type) = file_name.to_str() {
            for allow_file_type in allow_file_types.iter() {
                if allow_file_type == file_type {
                    true;
                    break;
                }
            }
        }
    }
    false
}

/// 读取文件
fn read_file(path: &str) {
    println!("read file {}", path)
}
