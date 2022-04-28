/// ### 按奇偶排序数组

/// ### 简单

/// ### 给你一个整数数组 nums，将 nums 中的的所有偶数元素移动到数组的前面，后跟所有奇数元素。

/// ### 返回满足此条件的 任一数组 作为答案。

/// ###

/// ### 示例 1：

/// ### 输入：nums = [3,1,2,4]
/// ### 输出：[2,4,3,1]
/// ### 解释：[4,2,3,1]、[2,4,1,3] 和 [4,2,1,3] 也会被视作正确答案。
/// ### 示例 2：

/// ### 输入：nums = [0]
/// ### 输出：[0]
/// ###

/// ### 提示：

/// ### 1 <= nums.length <= 5000
/// ### 0 <= nums[i] <= 5000
/// ### 通过次数75,205提交次数106,471

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/sort-array-by-parity
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        while left < right && nums[left] % 2 == 0 {
            left += 1;
        }
        while left < right && nums[right] % 2 == 1 {
            right -= 1;
        }
        if left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    nums
}
