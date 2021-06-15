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

