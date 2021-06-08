use http::ThreadPool;
use std::io::Result;
use std::net::TcpListener;
mod file_process;
mod cache;

fn main() -> Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:3303")?;
    let mut pool_thread = ThreadPool::new(4);
    cache::use_cache();
    for stream in tcp_listener.incoming() {
        let stream = stream?;
        pool_thread.execute(|| file_process::connect_handler(stream))
    }
    Ok(())
}
