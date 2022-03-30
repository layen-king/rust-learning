/// ## 寻找峰值

/// ### 中等

/// ### 峰值元素是指其值严格大于左右相邻值的元素。

/// ### 给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。

/// ### 你可以假设 nums\[-1\] = nums\[n\] = -∞ 。

/// ### 你必须实现时间复杂度为 O(log n) 的算法来解决此问题。

/// ###

/// ### 示例 1：

/// ### 输入：nums = \[1,2,3,1\]
/// ### 输出：2
/// ### 解释：3 是峰值元素，你的函数应该返回其索引 2。
/// ### 示例 2：

/// ### 输入：nums = \[1,2,1,3,5,6,4\]
/// ### 输出：1 或 5
/// ### 解释：你的函数可以返回索引 1，其峰值元素为 2；
/// ###      或者返回索引 5， 其峰值元素为 6。
/// ###

/// ### 提示：

/// ### 1 <= nums.length <= 1000
/// ### -231 <= nums\[i\] <= 231 - 1
/// ### 对于所有有效的 i 都有 nums\[i\] != nums\[i + 1\]
/// ### 通过次数129,191提交次数260,146

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/find-peak-element
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    let mut index = (nums.len() / 2) as i32;
    loop {
        if helper(&nums, index) > helper(&nums, index + 1)
            && helper(&nums, index) > helper(&nums, index - 1)
        {
            return index;
        } else {
            if helper(&nums, index) < helper(&nums, index + 1) {
                index += 1;
            } else {
                index -= 1;
            }
        }
    }
}
fn helper(nums: &Vec<i32>, i: i32) -> i32 {
    if i >= 0 && i < nums.len() as i32 {
        return nums[i as usize];
    }
    i32::MIN
}


pub fn find_peak_element1(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    let mut mid = nums.len() / 2 - 1;
    loop {
        let current = nums.get(mid).unwrap_or(&i32::MIN);
        let left = nums.get(mid - 1).unwrap_or(&i32::MIN);
        let right = nums.get(mid + 1).unwrap_or(&i32::MIN);
        if current > left && current > right {
            return mid as i32;
        }
        if current < left {
            mid -= 1;
        } else {
            mid += 1;
        }
    }
}


#[test]
fn test_find_peak_element() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let result = find_peak_element1(vec![1, 2, 1, 3, 5, 6, 4]);
    assert_eq!(result, 1);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("using time : {:?}", end - start);
}
