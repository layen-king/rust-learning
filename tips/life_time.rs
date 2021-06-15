/// 生命周期
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  // 必须使用生命周期表明
  if s1.len() > s2.len() {
      s1 // 此时无法推断生命周期,不在同一域内
  } else {
      s2
  }
}

pub fn run_life_time() {
  let s1 = String::from("rust");
  let s1_ptr = &s1;
  {
      let s2 = String::from("C");
      let longer = longer(s1_ptr, &s2);
      println!("{} is longer", longer)
  }
}
