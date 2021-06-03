use serde::{Deserialize, Serialize};
//use serde_json::Result;
use bufstream::BufStream;
use std::{fs::File, io::Read};
use std::{io::BufRead, net::TcpStream};

/// 配置项结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub root_dir: String,
    pub index: Vec<String>,
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
    // todo 读取流可能触发IO错误,待使用更优化方式解决
    buf.read_line(&mut req_string).unwrap();
    let _type = parse_url(&req_string)[0];
    let mut file_path = parse_url(&req_string)[1];
    println!("request url: {},config: {:?}", file_path, config);
    match _type {
        "GET" => {
            // 若在请求地址在配置项index字段内,直接读取root_dir文件
            for url in config.index.iter() {
                if url == file_path {
                    file_path = &config.root_dir;
                    break;
                }
            }
            read_file(file_path);
        }
        "POST" => {}
        "DELETE" => {}
        "PUT" => {}
        _ => {}
    }
}

/// 读取文件
fn read_file(path: &str) {
    println!("read file {}", path)
}
