/// ### 旋转函数

/// ### 中等

/// ### 给定一个长度为 n 的整数数组 nums 。

/// ### 假设 arrk 是数组 nums 顺时针旋转 k 个位置后的数组，我们定义 nums 的 旋转函数  F 为：

/// ### F(k) = 0 * arrk[0] + 1 * arrk[1] + ... + (n - 1) * arrk[n - 1]
/// ### 返回 F(0), F(1), ..., F(n-1)中的最大值 。

/// ### 生成的测试用例让答案符合 32 位 整数。

/// ###

/// ### 示例 1:

/// ### 输入: nums = [4,3,2,6]
/// ### 输出: 26
/// ### 解释:
/// ### F(0) = (0 * 4) + (1 * 3) + (2 * 2) + (3 * 6) = 0 + 3 + 4 + 18 = 25
/// ### F(1) = (0 * 6) + (1 * 4) + (2 * 3) + (3 * 2) = 0 + 4 + 6 + 6 = 16
/// ### F(2) = (0 * 2) + (1 * 6) + (2 * 4) + (3 * 3) = 0 + 6 + 8 + 9 = 23
/// ### F(3) = (0 * 3) + (1 * 2) + (2 * 6) + (3 * 4) = 0 + 2 + 12 + 12 = 26
/// ### 所以 F(0), F(1), F(2), F(3) 中的最大值是 F(3) = 26 。
/// ### 示例 2:

/// ### 输入: nums = [100]
/// ### 输出: 0
/// ###

/// ### 提示:

/// ### n == nums.length
/// ### 1 <= n <= 105
/// ### -100 <= nums[i] <= 100
/// ### 通过次数14,170提交次数30,115

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/rotate-function
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
    // 解释 f(0) = 0 + nums[1] * 1 + nums[2] *2 + nums[3] * 3 ... + nums[n-1] * n-1
    // f(1) = 0 + nums[0] * 1 + nums[1] * 2 + nums[2] * 3 + ... nums[n-2] * n-1
    // 推断 f(k) = f(k-1) + numsSum - n * nums[n -k]
    let (mut f, n, nums_sum) = (0, nums.len(), { nums.iter().fold(0, |acc, x| acc + x) });
    nums.iter()
        .enumerate()
        .for_each(|(i, x)| f = f + x * i as i32);
    let mut res = f;
    for i in 0..n {
        f = f + nums_sum - (n as i32) * nums[n - i - 1];
        res = f.max(res)
    }
    res
}

#[test]
fn test_max_rotate_function() {
    let res = max_rotate_function(vec![4, 3, 2, 6]);
    assert_eq!(res, 26)
}
