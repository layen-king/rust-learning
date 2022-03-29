#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
#[allow(dead_code)]
impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x2 - x1) * (y2 - y1)).abs()
    }
}

#[allow(dead_code)]
// `Pair` 拥有资源：两个堆分配的整型
struct Pair(Box<i32>, Box<i32>);

#[allow(dead_code)]
impl Pair {
    // 这个方法会 “消耗” 调用者的资源
    // `self` 为 `self: Self` 的语法糖
    fn destroy(self) {
        // 解构 `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` 和 `second` 离开作用域后释放
    }
}

#[test]
pub fn test() {
    let p1 = Point::origin();
    let p2 = Point::new(2.0, 2.0);
    let rect = Rectangle { p1, p2 };
    println!("{}", rect.area());
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy()
}
