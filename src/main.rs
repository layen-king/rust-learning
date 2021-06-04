mod utils;
mod cache;
mod mime_type;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let result = utils::read_config();
    if let Ok(config) = result {
        let ip_port = format!("{}:{}", config.ip, config.port);
        let listener = TcpListener::bind(ip_port)?;
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                utils::connect_handler(stream, config.clone())?;
            } else {
                println!("error stream")
            }
        }
    }

    Ok(())
}
