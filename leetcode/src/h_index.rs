/// ## H 指数 II
/// ### 输入: citations = [0,1,3,5,6]
/// ### 输出: 3
/// ### 解释: 给定数组表示研究者总共有 5 篇论文，每篇论文相应的被引用了 0, 1, 3, 5, 6 次。
/// ###    由于研究者有 3 篇论文每篇至少被引用了 3 次，其余两篇论文每篇被引用不多于 3 次，所以她的 h 指数是 3。
/// #### 说明:
/// > 如果 h 有多有种可能的值 ，h 指数是其中最大的那个。
#[allow(dead_code)]
pub fn h_index(citations: Vec<i32>) -> i32 {
  // todo 此版本存在问题
  let len = citations.len();
  let mut left = 0;
  let mut right = len - 1;
  while right > left {
      let mid = left + (right - left) / 2 >> 0;
      if (citations[mid] as usize) < len - mid {
          left = mid + 1
      } else {
          right = mid - 1
      }
  }
  (len - left) as i32
}

#[allow(dead_code)]
pub fn h_index_1(citations: Vec<i32>) -> i32 {
  let len = citations.len();
  let mut l = 0;
  let mut r = len;
  let mut ret = 0;
  while l < r {
      let m = (l + r) / 2;
      if citations[m] as usize <= len - m {
          ret = ret.max(citations[m]);
          l = m + 1;
      } else {
          ret = ret.max((len - m) as i32);
          r = m;
      }
  }
  ret
}
