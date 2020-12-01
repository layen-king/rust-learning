#[derive(Debug)]
/// 结构体
pub struct Rect {
  pub width: u32,
  pub height: u32,
}
pub fn rect() {
  let rect1 = Rect {
    width: 30,
    height: 50,
  };
  println!("rect1 is {:?}", rect1)
}

/// 结构体方法
impl Rect {
  pub fn area(&self) -> u32 {
    return self.width * self.height;
  }
}

/// 结构体函数
impl Rect {
  pub fn create(width: u32, height: u32) -> Rect {
    return Rect { width, height };
  }
}
