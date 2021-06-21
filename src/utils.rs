use std::collections::HashMap;
// use anyhow;
use std::io::{Read, Result};
use std::net::TcpStream;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Request<'a> {
    method: &'a str,
    url: &'a str,
}

impl Request<'_> {
    fn new<'a>(method: &'a str, url: &'a str) -> Request<'a> {
        Request { method, url }
    }
}

fn parse_url(requst: &str) -> Vec<&str> {
    let v: Vec<&str> = requst.split("\r\n").collect();
    let mut map: HashMap<&str, &str> = HashMap::new();
    for str in v.iter() {
        if str.contains(":") {
            let str = *str;
            let kv = str.split(":").collect::<Vec<&str>>();
            map.insert(kv[0], kv[1]);
        }
    }
    println!("map: {:?}", map);
    v
}

/// 处理tcp流
pub fn handle_connect(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 1024];
    stream.read(&mut buf)?;
    let request = String::from_utf8_lossy(&buf);
    let v = parse_url(&request);
    let request = Request::new(v[0], v[1]);
    println!("{:?}", request);
    Ok(())
}
