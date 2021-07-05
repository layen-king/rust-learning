/// 执行泛型和trait主函数,en: perform generic and trait main functions
pub fn run_trait() {
  use_option();
  use_trait();
}

use std::fmt::Debug;
/// 对于option的使用，en: for the use of option
fn match_option<T: Debug>(o: Option<T>) {
  match o {
      Some(i) => println!("value is {:?}", i),
      None => println!("nothing found"),
  }
}
/// option实例
fn use_option() {
  let a = Some(3);
  match_option(a);
  let b: Option<u32> = None;
  match_option(b);
}

struct Duck;
struct Pig;
/// 某一特征，既接口
trait Fly {
  fn fly(&self) -> bool;
}

/// 实现接口
impl Fly for Duck {
  fn fly(&self) -> bool {
      true
  }
}

impl Fly for Pig {
  fn fly(&self) -> bool {
      false
  }
}

/// 以特征为泛型写法
fn static_fly<T: Fly>(s: T) -> bool {
  s.fly()
}

/// dyn 特征对象前缀  en :dyn is a prefix of a trait object's type.
fn dyn_fly(s: &dyn Fly) -> bool {
  s.fly()
}

use std::fmt::{Formatter, Result};
struct Point {
  x: i32,
  y: i32,
}
impl Debug for Point {
  fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "Point : {{ x :{}, y: {} }}", self.x, self.y)
  }
}

// trait 继承

trait Page {
  fn set_page(&self, p: i32) {
      println!("page default: {}", p);
  }
}
trait PrePage {
  fn set_pre_page(&self, p: i32) {
      println!("pre page default: {}", p);
  }
}
trait Paginate: Page + PrePage {
  fn set_skip(&self, num: i32) {
      println!("skip is :{}", num);
  }
}
// 实现组合继承 为所有拥有Page和PagePre的类型实现Paginate
impl<T: Page + PrePage> Paginate for T {}

// 泛型约束
// TODO: 待学习
use std::ops::Add;
#[allow(dead_code)]
fn sum<T: Add<T, Output = T>>(a: T, b: T) -> T {
  a + b
}

fn use_trait() {
  let pig = Pig;
  let duck = Duck;
  let p = static_fly::<Pig>(pig);
  println!("pig can flv :? {}", p);

  let d = static_fly::<Duck>(duck);
  println!("duck can flv :? {}", d);

  dyn_fly(&Duck);

  let point = Point { x: 0, y: 0 };
  println!("point :{:?}", point);
}
