trait Comparable {
  fn compare(&self, object: &Self) -> i8;
}

fn max<T: Comparable>(arr: &[T]) -> &T {
  let mut max_index = 0;
  let mut i = 0;
  while i < arr.len() {
    if arr[i].compare(&arr[max_index]) > 0 {
      max_index = i
    }
    i += 1;
  }
  return &arr[max_index];
}

impl Comparable for f64 {
  fn compare(&self, n: &f64) -> i8 {
    if &self > &n {
      1
    } else if &self == &n {
      0
    } else {
      -1
    }
  }
}

pub fn get_max_num() {
  let arr = [1.0, 2.0, 3.0, 4.0, 2.0, 10.0, 3.0];
  println!("max is {}", max(&arr))
}
