use std::f64::{INFINITY, NEG_INFINITY};
/// ## 寻找两个正序数组的中位数

/// ### 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
/// ### 示例 1：

/// ### 输入：nums1 = [1,3], nums2 = [2]
/// ### 输出：2.00000
/// ### 解释：合并数组 = [1,2,3] ，中位数 2
/// ### 示例 2：

/// ### 输入：nums1 = [1,2], nums2 = [3,4]
/// ### 输出：2.50000
/// ### 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
/// ### 示例 3：

/// ### 输入：nums1 = [0,0], nums2 = [0,0]
/// ### 输出：0.00000
/// ### 示例 4：

/// ### 输入：nums1 = [], nums2 = [1]
/// ### 输出：1.00000
/// ### 示例 5：

/// ### 输入：nums1 = [2], nums2 = []
/// ### 输出：2.00000
#[allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {
        process(nums2, nums1)
    } else {
        process(nums1, nums2)
    }
}

fn process(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let l1 = nums1.len();
    let l2 = nums2.len();
    let mut left = 0;
    let mut right = l1;

    let mut m1 = 0;
    let mut m2 = 0;

    while left <= right {
        let i = left + (((right - left) / 2) as f64).ceil() as usize;
        let j = (((l1 + l2 + 1) / 2 - i) as f64).ceil() as usize;
        // println!("i:{},j:{}", i, j);

        let max_left_1 = if i == 0 {
            NEG_INFINITY
        } else {
            nums1[i - 1] as f64
        };
        let min_right_1 = if i == l1 {
            INFINITY
        } else {
            nums1[i] as f64
        };
        let max_left_2 = if j == 0 {
            NEG_INFINITY
        } else {
            nums2[j - 1] as f64
        };
        let min_right_2 = if j == l2 {
            INFINITY
        } else {
            nums2[j] as f64
        };
        // println!("max_left_1:{}", max_left_1);
        // println!("max_right_2:{}", min_right_2);
        if max_left_1 <= min_right_2 {
            m1 = std::cmp::max(max_left_1 as isize, max_left_2 as isize);
            m2 = std::cmp::min(min_right_1 as isize, min_right_2 as isize);
            left += 1;
        } else {
            right -= 1;
        }
    }

    // println!("m1,m2,{},{}",m1,m2);

    if (l1 + l2) % 2 == 0 {
        ((m1 + m2) as f64) / (2 as f64)
    } else {
        m1 as f64
    }
}

#[test]
fn process_test() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let r = process(nums1, nums2);
    assert_eq!(r, 2.00000);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4, 5];
    let r = process(nums1, nums2);
    assert_eq!(r, 3.0000);

    let nums1 = vec![1];
    let nums2 = vec![2];
    let r = process(nums1, nums2);
    assert_eq!(r, 1.50000);
}
