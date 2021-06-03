/// 函数引用
pub fn quote() {
  let s1 = String::from("hello");
  let s2 = &s1;
  println!("s2 is {},s1 is {}", s2, s1)
}

/// 可变函数引用x
pub fn var_quote() {
  let mut s1 = String::from("run");
  let s2 = &mut s1;
  s2.push_str("kkkk");
  println!("s1 is {} ", s1)
}
