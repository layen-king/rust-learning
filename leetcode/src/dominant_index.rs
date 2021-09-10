// 至少是其他数字两倍的最大数

// 给你一个整数数组 nums ，其中总是存在 唯一的 一个最大整数 。

// 请你找出数组中的最大元素并检查它是否 至少是数组中每个其他数字的两倍 。如果是，则返回 最大元素的下标 ，否则返回 -1 。

// 示例 1：

// 输入：nums = [3,6,1,0]
// 输出：1
// 解释：6 是最大的整数，对于数组中的其他整数，6 大于数组中其他元素的两倍。6 的下标是 1 ，所以返回 1 。
// 示例 2：

// 输入：nums = [1,2,3,4]
// 输出：-1
// 解释：4 没有超过 3 的两倍大，所以返回 -1 。
// 示例 3：

// 输入：nums = [1]
// 输出：0
// 解释：因为不存在其他数字，所以认为现有数字 1 至少是其他数字的两倍。
pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let mut index = i32::MIN;
    let mut max = i32::MIN;
    for i in 0..nums.len() {
        if nums[i] > max {
            index = i as i32;
            max = nums[i];
        }
    }

    let iter = nums.into_iter().find(|&x| x != max && x * 2 > max);
    if iter.is_none() {
        index
    } else {
        -1
    }
}
// use std::collections::BinaryHeap;
/// ## 使用最大堆
// pub fn dominant_index1(nums: Vec<i32>) -> i32 {
//     if nums.len() == 1 {
//         return 0i32;
//     }
//     let mut index = 0;
//     let mut max = i32::MIN;
//     let mut heap = BinaryHeap::new();
//     for (i, n) in nums.iter().enumerate() {
//         if n > &max {
//             index = 1;
//             max = *n;
//         }
//         heap.append(n);
    
//     heap.peek();
//     let second = heap.peek().unwrap();
//     if &max >= second * 2 {
//         index
//     } else {
//         -1
//     }
//     todo!()
// }

#[test]
fn test_dominant_index() {
    let result = dominant_index(vec![1, 2, 3, 4]);
    assert_eq!(result, -1);
    let result = dominant_index(vec![3, 6, 1, 0]);
    assert_eq!(result, 1);
    let result = dominant_index(vec![0, 0, 1, 2]);
    assert_eq!(result, 3);
}
