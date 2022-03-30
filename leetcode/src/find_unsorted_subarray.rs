/// ## 最短无序连续子数组

/// ### 给你一个整数数组 nums ，你需要找出一个 连续子数组 ，如果对这个子数组进行升序排序，那么整个数组都会变为升序排序。

/// ### 请你找出符合题意的 最短 子数组，并输出它的长度。

/// ###

/// ### 示例 1：

/// ### 输入：nums = \[2,6,4,8,10,9,15\]
/// ### 输出：5
/// ### 解释：你只需要对 \[6, 4, 8, 10, 9\] 进行升序排序，那么整个表都会变为升序排序。
/// ### 示例 2：

/// ### 输入：nums = \[1,2,3,4\]
/// ### 输出：0
/// ### 示例 3：

/// ### 输入：nums = \[1\]
/// ### 输出：0
/// ###

/// ### 提示：

/// ### 1 <= nums.length <= 104
/// ### -105 <= nums\[i\] <= 105
pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let mut nums_sorted = nums.clone();
    nums_sorted.sort();
    let [mut left, mut right] = [0, nums.len() - 1];
    while left < right {
        if &nums[left] != &nums_sorted[left] {
            break;
        }
        left += 1;
    }
    while left < right {
        if &nums[right] != &nums_sorted[right] {
            break;
        }
        right -= 1;
    }
    if left == right {
        0 as i32
    } else {
        (right - left + 1) as i32
    }
}

#[test]
fn test_find_unsorted_subarray() {
    let result = find_unsorted_subarray(vec![1, 2, 3, 4]);
    assert_eq!(result, 0);
    let result = find_unsorted_subarray(vec![1]);
    assert_eq!(result, 0);
    let result = find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]);
    assert_eq!(result, 5);
}
