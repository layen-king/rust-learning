/// 流程控制
pub fn rust_if_else() {
  let n = 13;
  let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
  assert_eq!(big_n, 6); // true,因为推断为i32类型，会截取6.5为6
  rust_for();
  rust_match();
  rust_if_let();
}

/// for循环
fn rust_for() {
  for n in 1..101 {
      // 1..101 一种range
      if n % 15 == 0 {
          println!("fizzbuzz")
      } else if n % 3 == 0 {
          println!("fizz")
      } else if n % 5 == 0 {
          println!("buzz")
      } else {
          println!("{}", n)
      }
  }
}

/// match必须穷举所有可能性
fn rust_match() {
  let number = 42;
  match number {
      0 => println!("number is 0"),
      1 | 5 | 7 | 13 => println!("number is 1 or 5 or 7 or 13"), //可能性
      n @ 42 => println!("number is {}", n),                     // 绑定变量
      _ => println!("number is other"),                          // 其余所有可能,必须有
  }
}

/// if let简化 操作
fn rust_if_let() {
  let mut v = vec![1, 2, 3, 4, 5];
  while let Some(x) = v.pop() {
      // 使用if let 比较,可以直接用Some使用
      println!("v is {}", x)
  }
  // 不使用if let简化
  // 因为pop会永久改变向量
  loop {
      match v.pop() {
          Some(x) => println!("use match: {}", x), // 若不使用if let ,match必须匹配接收到所有的值
          None => break,
      }
  }
}
