mod utils;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let result = utils::read_config();
    if let Ok(config) = result {
        let ip_port = format!("{}:{}", config.ip, config.port);
        let listener = TcpListener::bind(ip_port)?;
        for stream in listener.incoming() {
            match stream {
                Ok(s) => utils::connect_handler(s, config.clone()),
                _ => {
                    println!("error stream")
                }
            }
        }
    }

    Ok(())
}
