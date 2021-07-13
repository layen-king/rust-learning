use anyhow::Result;
use std::net::TcpListener;
mod make_headers;
mod utils;

fn main() -> Result<()> {
    let m :i32 = 0;
    assert_eq!( m==0,true);
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
