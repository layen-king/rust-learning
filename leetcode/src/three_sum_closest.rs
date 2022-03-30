/// ## 最接近的三数之和
/// ### 中等

/// ### 给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，
/// 使得它们的和与 target 最接近。返回这三个数的和。假定每组输入只存在唯一答案。

/// ###

/// ### 示例：

/// ### 输入：nums = \[-1,2,1,-4\], target = 1
/// ### 输出：2
/// ### 解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
/// ###

/// ### 提示：

/// ### 3 <= nums.length <= 10^3
/// ### -10^3 <= nums\[i\] <= 10^3
/// ### -10^4 <= target <= 10^4

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/3sum-closest
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums_copy = nums.clone();
    nums_copy.sort();
    let mut result = 10i32.pow(3);
    for i in 0..nums.len() {
        let [mut left, mut right] = [i + 1, nums.len() - 1];
        while left < right {
            let sum = nums_copy[i] + nums_copy[left] + nums_copy[right];
            if (result - target).abs() > (sum - target).abs() {
                result = sum;
            }
            if target > sum {
                left += 1;
            } else if target < sum {
                right -= 1;
            } else {
                return target;
            }
        }
    }
    result as i32
}

#[test]
fn test_three_sum_closest() {
    let result = three_sum_closest(vec![-1, 2, 1, -4], 1);
    assert_eq!(result, 2)
}
