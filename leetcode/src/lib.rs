/// ## 两数之和
/// ### 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，
/// ### 并返回//// 它们的数组下标。
/// ### 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
/// ### 你可以按任意顺序返回答案。
/// ### 暴力解法
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![0, 1];
}
use std::collections::HashMap;
/// ## 两数之和
/// ### 使用HashMap
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

/// ## H 指数 II
/// ### 输入: citations = [0,1,3,5,6]
/// ### 输出: 3
/// ### 解释: 给定数组表示研究者总共有 5 篇论文，每篇论文相应的被引用了 0, 1, 3, 5, 6 次。
/// ###    由于研究者有 3 篇论文每篇至少被引用了 3 次，其余两篇论文每篇被引用不多于 3 次，所以她的 h 指数是 3。
/// #### 说明:
/// > 如果 h 有多有种可能的值 ，h 指数是其中最大的那个。
pub fn h_index(citations: Vec<i32>) -> i32 {
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
