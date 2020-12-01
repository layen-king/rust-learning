/// 切片
pub fn slice_arr() {
  let arr = [1, 2, 3, 4, 5, 6, 7, 8, 100, 1000];
  let part = &arr[..5];
  for i in part.iter() {
    println!("{}", i)
  }
}
