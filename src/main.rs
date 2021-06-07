use http::ThreadPool;
use std::io::Result;
use std::net::TcpListener;
mod file_process;

fn main() -> Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:3303")?;
    let mut pool_thread = ThreadPool::new(4);
    for stream in tcp_listener.incoming().take(2) {
        let stream = stream?;
        pool_thread.execute(|| file_process::connect_handler(stream))
    }
    Ok(())
}
