/// 获取索引
pub fn get_index(arr: [&str; 3], s: &str) -> usize {
  let mut i = 0;
  let location = loop {
    let ch = arr[i];
    if s == ch {
      break i;
    }
    i += 1;
  };
  return location;
}
