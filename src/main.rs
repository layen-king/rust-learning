use anyhow::Result;
use std::net::TcpListener;
mod utils;
use mime_types;

fn main() -> Result<()> {
    mime_types::test();
    // practice::grades::grades();
    let listener = TcpListener::bind("127.0.0.1:3303")?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            println!("stream : {:?}", stream);
            let s = utils::handle_connect(stream);
            match s {
                Ok(_) => {}
                Err(e) => {
                    println!("error : {:?}", e)
                }
            }
        }
    }
    Ok(())
}
