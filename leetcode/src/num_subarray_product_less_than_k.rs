/// ## 乘积小于 K 的子数组

/// ### 中等

/// ### 给你一个整数数组 nums 和一个整数 k ，请你返回子数组内所有元素的乘积严格小于 k 的连续子数组的数目。
/// ###

/// ### 示例 1：

/// ### 输入：nums = [10,5,2,6], k = 100
/// ### 输出：8
/// ### 解释：8 个乘积小于 100 的子数组分别为：[10]、[5]、[2],、[6]、[10,5]、[5,2]、[2,6]、[5,2,6]。
/// ### 需要注意的是 [10,5,2] 并不是乘积小于 100 的子数组。
/// ### 示例 2：

/// ### 输入：nums = [1,2,3], k = 0
/// ### 输出：0
/// ###

/// ### 提示:

/// ### 1 <= nums.length <= 3 * 104
/// ### 1 <= nums[i] <= 1000
/// ### 0 <= k <= 106
/// ### 通过次数49,282提交次数107,552

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/subarray-product-less-than-k
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let (mut cur, mut res, mut j) = (1, 0, 0);
    for i in 0..nums.len() {
        cur *= nums[i];
        while j <= i && cur >= k {
            cur /= nums[j];
            j += 1;
        }
        res += j - i + 1;
    }
    res as i32
}

#[test]
fn test_num_subarray_product_less_than_k() {
    let result = num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100);
    assert_eq!(result, 8)
}
