use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Cache {
    /// 文件映射列表
    pub file_map: HashMap<String, String>,
    /// 需要缓存
    pub has_cache: bool,
}

#[allow(dead_code)]
impl Cache {
    pub fn new(file_map: HashMap<String, String>, has_cache: bool) -> Cache {
        Cache {
            file_map,
            has_cache,
        }
    }
    /// 生成返回文件
    pub fn make_response(&mut self, path: &str) -> String {
        if self.has_cache {
            let get_file = self.file_map.get(path);
            if let Some(file) = get_file {
                return String::from(file);
            } else {
                let response = self.read_file(path);
                let _r = response.clone();
                self.file_map.insert(String::from(path), response);
                return _r;
            }
        } else {
            let response = self.read_file(path);
            return response;
        }
    }
    /// 读取文件
    fn read_file(&self, path: &str) -> String {
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
}
