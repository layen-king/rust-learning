#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use std::net::TcpListener;
mod make_headers;
mod utils;
#[allow(unused_imports)]
use del_repeat;
#[allow(unused_imports)]
use leetcode;
#[allow(unused_imports)]
use rust_course;
fn main() -> Result<()> {
    del_repeat::repeat::ask_user();
    // rust_course::async_start::start();
    // rust_course::async_start::async_start();
    rust_course::async_future_excuting::test_execute_and_spwan();
    rust_course::async_await::start();
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

// fn main() {
//     del_repeat::repeat::ask_user();
// }
