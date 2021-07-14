/// ## 两数之和
/// ### 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，
/// ### 并返回//// 它们的数组下标。
/// ### 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
/// ### 你可以按任意顺序返回答案。
/// ### 暴力解法
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  for i in 0..nums.len() {
      for j in i + 1..nums.len() {
          if nums[i] + nums[j] == target {
              return vec![i as i32, j as i32];
          }
      }
  }
  panic!("not found")
}
use std::collections::HashMap;
/// ## 两数之和
/// ### 使用HashMap
#[allow(dead_code)]
pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut map = HashMap::with_capacity(nums.len());
  for i in 0..nums.len() {
      if let Some(n) = map.get(&(target - nums[i])) {
          return vec![*n as i32, i as i32];
      }
      map.insert(nums[i], i);
  }
  panic!("not found")
}