use anyhow::Result;
use std::net::TcpListener;
mod make_headers;
mod utils;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3303")?;
    for stream in listener.incoming() {
        let stream = stream?;
        match utils::handle_connect(stream) {
            Ok(_) => {}
            Err(err) => println!("error: {:?}", err),
        }
    }
    Ok(())
}
