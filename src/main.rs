mod cache;
mod read_config;
mod utils;
use std::collections::HashMap;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let config = read_config::read_config()?;
    let mut file_process = cache::Cache::new(HashMap::new(), config.cache);
    println!("file_process is {:?}", file_process);
    let ip_port = format!("{}:{}", &config.ip, &config.port);
    let listener = TcpListener::bind(ip_port)?;
    for stream in listener.incoming() {
        let stream = stream?;
        utils::connect_handler(stream, &config, &mut file_process)?;
    }
    Ok(())
}
