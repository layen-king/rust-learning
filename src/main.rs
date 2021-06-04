mod read_config;
mod cache;
mod utils;
use std::net::TcpListener;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let config = read_config::read_config()?;
    let file_process = cache::Cache::new(HashMap::new(),config.cache);
    println!("file_process is {:?}",file_process);
    let ip_port = format!("{}:{}", &config.ip, &config.port);
    let listener = TcpListener::bind(ip_port)?;
    for stream in listener.incoming() {
        let stream = stream?;
        utils::connect_handler(stream, &config)?;
    }
    Ok(())
}
