/// 智能指针Box
pub fn run_box() {
  #[derive(PartialEq, Debug)]
  struct Point {
      x: f64,
      y: f64,
  }
  // 使用Box装箱,分配给堆内存，en:use box to box and allocate to heap memory
  let box_point = Box::new(Point { x: 0.0, y: 0.0 });
  // 使用 * 解除引用,返回结构体自身, en: use * to dereference and return to the structure
  let unbox_point = *box_point;
  // 无须手动始放内存， en: no need to manually start the momery
  println!("box is {:?}", unbox_point);
}
