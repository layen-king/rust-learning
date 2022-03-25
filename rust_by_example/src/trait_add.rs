// 泛型扩展
use std::ops::Add;

/// ## 结构体point
/// ### x,y成员必须实现Add trait
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>; // 执行返回类型为Point

    fn add(self, p: Point<T>) -> Self::Output {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

#[allow(dead_code)]
pub fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

#[test]
fn test() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = add(p1, p2);
    print!("{:#?}", p3);
    assert_eq!(p3, Point { x: 4, y: 6 });
}
