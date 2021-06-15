use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;

#[macro_use]

lazy_static! {
    static ref MAP: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

/// 读取缓存
pub fn read(path: &str) -> String {
    if let Ok(read) = MAP.try_read() {
        let str = String::from("");
        let content = read.get(path).unwrap_or(&str);
        content.to_string()
    } else {
        String::from("")
    }
}

/// 写入缓存
pub fn write(path: &str, content: &str) -> bool {
    if let Ok(mut write) = MAP.try_write() {
        if let Some(_) = write.insert(path.to_string(), content.to_string()) {
            return true;
        }
    }
    false
}
