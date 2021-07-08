use anyhow::Result;
use std::net::TcpListener;
mod utils;
mod make_headers;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3303")?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            utils::handle_connect(stream)?;
        }
    }
    Ok(())
}
