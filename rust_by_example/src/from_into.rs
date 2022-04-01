use std::convert::From;

#[derive(Debug)]
struct Number {
    #[allow(dead_code)]
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

#[test]
fn test(){
  let k = Number::from(30);
  println!("{:?}",k);
  let n:Number = 310.into();
  println!("{:?}",n);
}
