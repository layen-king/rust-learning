/// 可变变量与借用
pub fn rust_mut() {
  let a = [1, 2, 3];
  let b = &a[..];
  println!("{:p}", b); // 宏定义指针地址
  let mut c = vec![1, 2, 3];
  let d = &mut c;
  d.push(4);
  println!("{:?}", d); // vec打印必须先格式化
  let e = &42;
  assert_eq!(42, *e); // 取指针值
}

/// 作用域
pub fn rust_scope() {
  let v = "hello rust";
  assert_eq!(v, "hello rust");
  let v = "hello world";
  assert_eq!(v, "hello world");
  {
      let v = "hello rust";
      assert_eq!(v, "hello rust");
  }
  assert_eq!(v, "hello");
  rust_2();
  rust_3();
  mut_1();
}

/// 使用闭包与普通函数
fn rust_2() {
  let out = 42;
  fn add(i: i32, j: i32) -> i32 {
      i + j
  }
  let closure_a = |i: i32, j: i32| -> i32 { i + j + out }; // 闭包捕获外部变量
  let closure_i = |i: i32, j: i32| -> i32 { i + j + out };
  assert_eq!(3, add(1, 2));
  assert_eq!(45, closure_a(1, 2));
  assert_eq!(45, closure_i(1, 2))
}

/// 函数,闭包作为参数
fn rust_3() {
  fn closure_m<F: Fn() -> i32>(op: F) -> i32 {
      op()
  }
  let a = 2;
  let b = 3;
  assert_eq!(closure_m(|| { a + b }), 5);
  assert_eq!(closure_m(|| a * b), 6);

  /// 闭包返回必须用move
  fn closure_2() -> impl Fn(i32) -> i32 {
      let i = 3;
      move |j| i * j
  }
  assert_eq!(12, closure_2()(4));
}

use std::cell::Cell;
///内部可变性cell
fn mut_1() {
  struct Foo {
      x: u32,
      y: Cell<u32>,
  }
  let foo = Foo {
      x: 1,
      y: Cell::new(2),
  };
  foo.y.set(10); // Cell使用set设置数据,使用get获取数据
  println!("x is:{},y is :{}", foo.x, foo.y.get());

}
