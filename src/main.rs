use http::ThreadPool;
use std::io::Result;
use std::net::TcpListener;
mod cache;
mod file_process;

fn main() -> Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:3303")?;
    let mut pool_thread = ThreadPool::new(4);
    cache::use_cache();

    for stream in tcp_listener.incoming().filter_map(|s| s.ok()) {
        pool_thread.execute(|| file_process::handle_connect(stream));
    }

    /* for stream in tcp_listener.incoming().take(2) {
        let stream = stream?;
        pool_thread.execute(|| file_process::handle_connect(stream))
    } */
    Ok(())
}
