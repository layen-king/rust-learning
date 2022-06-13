use anyhow::Result;
use std::net::TcpListener;
mod make_headers;
mod utils;
#[allow(unused_imports)]
use leetcode;
#[allow(unused_imports)]
use rust_course;
fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3303")?;
    println!("http is listening on 127.0.0.1:3303");
    for stream in listener.incoming() {
        let stream = stream?;
        match utils::handle_connect(stream) {
            Ok(_) => {}
            Err(err) => println!("error: {:?}", err),
        }
    }
    Ok(())
}
