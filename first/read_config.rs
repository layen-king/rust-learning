use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

/// 配置项结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// 配置的ip,127.0.0.1
    pub ip: String,
    /// 监听的端口号,3303
    pub port: String,
    /// 是否开启缓存
    pub cache: bool,
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
