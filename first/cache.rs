#[allow(dead_code)]
/// ## 读取文件
/// #### [file_path]: 文件路径
/// #### [use_cache]: 是否使用缓存
/// #### [result]: 返回文件字符串,已经处理错误
pub fn read_file(file_path: &str, use_cache: bool) -> String {
    println!("file_path is {}", file_path);
    println!("use_cache is {}", use_cache);
    String::from("cache")
}

#[allow(dead_code)]
/// ## 生成http相应内容
/// #### [content]: 读取的文件字符串
fn make_response(content: String) -> String {
    println!("content is: {}", content);
    String::from("cache")
}
