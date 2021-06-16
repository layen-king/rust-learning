/// 内建char
pub fn string_1() {
  assert_eq!(true, 'f'.is_digit(16)); // 判断是否为16进制
  assert_eq!(Some(15), 'f'.to_digit(16)); // 转换为16进制
  assert_eq!(true, 'a'.is_lowercase()); //判断是否小写
  assert_eq!(true, 'A'.is_uppercase()); // 判断是否大写

  // 字符串处理 字符与字节
  let s = String::from("I am learn rust");
  let mut charts = s.chars();
  assert_eq!(Some('I'), charts.next());

  let mut bytes = s.bytes();
  assert_eq!(Some(1), bytes.next());
}

/// 字符串查找
fn string_2() {
  let str = "bBah jkderfb";
  assert_eq!(str.contains("a"), true);
  assert_eq!(str.starts_with("b"), true);
  assert_eq!(str.ends_with("b"), true);
  assert_eq!(str.find("b"), Some(0));
  assert_eq!(str.rfind("b"), Some(0));
  assert_eq!(str.find("m"), None);
  assert_eq!(str.find(char::is_whitespace), Some(4));
  assert_eq!(str.find(char::is_lowercase), Some(1));
}


/// 字符串分割
fn string_3(){
  let s = "abcdefg中文配置hhh";
  let v = s.split(|c|(c as u32) >= (0x4e00 as u32) && (c as u32)<=(0x9FA5 as u 32));
  assert_eq!(v,["abcdefg","hhh"]);

  let v = "aaa bbb ccc ddd eee".splitn(3,' ').collect::Vec<&str>();
  assert_eq!(v,["aaa","bbb","ccc ddd eee"]);
}

/// 捕获匹配
fn string_4(){
  let s = "abcXXXefgXXXopq".matches("abc").collect::Vec<&str>();
  assert_eq!(["abc"],s);
  assert_eq!("abc".replace("a", "b"),"bbc");
  
}
