use std::collections::HashMap;

#[derive(Debug)]
struct FileCache {
    file_cache: HashMap<String, String>,
}

impl FileCache {
    pub fn new(file_cache: HashMap<String, String>) -> FileCache {
        FileCache { file_cache }
    }
    /// 缓存获取文件
    pub fn get_file(&mut self, path: &str) -> String {
        let file = self.file_cache.get(path);
        match file {
            Some(file) => file.to_string(),
            None => String::from(""),
        }
    }
    /// 设置缓存
    pub fn set_file_cache(&mut self, key: &str, value: &str) {
        self.file_cache.insert(key.to_string(), value.to_string());
    }
}

#[allow(dead_code)]
pub fn use_cache() {
    let map = HashMap::new();
    let mut file_cache = FileCache::new(map);
    let path = String::from("/dev");
    let value = String::from("100");
    file_cache.set_file_cache(&path, &value);
    let file = file_cache.get_file(&path);
    println!("file: {:?}", file);
}
