use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

/// 处理流
pub fn connect_handler(mut stream: TcpStream) {
    println!("stream: {:?}", stream);
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap_or_else(|err| {
        println!("write error: {}", err);
        0
    });
    let req_str = String::from_utf8_lossy(&buf[..]);
    let req_type = paser_req(&req_str)[0];
    let req_url = paser_req(&req_str)[1];
    let content = read_file(req_url);
    stream.write(content.as_bytes()).unwrap_or_else(|err| {
        println!("write error: {}", err);
        0
    });
    stream.flush().unwrap_or_else(|err| {
        println!("flush error: {}", err);
        ()
    });
    println!("req_type: {:?}", req_type);
    println!("req_url: {:?}", req_url);
    // println!("content: {:?}", content);
}

/// 解析请求文件
fn paser_req(request: &str) -> Vec<&str> {
    let v: Vec<&str> = request.split(' ').collect();
    v
}

/// 读取文件
fn read_file(path: &str) -> String {
    if let Ok(mut file) = File::open(path) {
        let mut file_content = String::new();
        // todo 若文件过大,使用文件流发送
        file.read_to_string(&mut file_content).ok();
        // todo 根据请求类型,生成内容
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            file_content.len(),
            file_content
        )
    } else {
        let file_content = String::from("404 NotFound");
        format!(
            "HTTP/1.1 404 NotFound\r\nContent-Length: {}\r\n\r\n{}",
            file_content.len(),
            file_content
        )
    }
}
