use std::fmt;


#[derive(Debug)]
struct MinMax(i64,i64);

impl fmt::Display for MinMax{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    let max = if self.0 >= self.1 {self.0} else {self.1};
    let min = if self.0 < self.1 {self.0} else {self.1};
    write!(f, "min is :{},max is :{}",max, min)
  }
}

struct List(Vec<i32>);
impl fmt::Display for List{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let v = &self.0;
    write!(f, "[")?;
    for (count,v) in v.iter().enumerate() {
      write!(f,"{}:{};",count,v)?;
    }
    write!(f, "]")
  }
}


#[test]
fn test_print_display(){
  let min_max = MinMax(1,2);
  println!("{}",min_max);
  println!("{:?}",min_max);
  let list = List(vec![1,3,4,5,6]);
  println!("{}",list);
}
