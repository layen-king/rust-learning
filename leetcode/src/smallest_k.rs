/// ## 最小K个数
/// ### 中等
/// ### 设计一个算法，找出数组中最小的k个数。以任意顺序返回这k个数均可。

/// ### 示例：

/// ### 输入： arr = \[1,3,5,7,2,4,6,8\], k = 4
/// ### 输出： \[1,2,3,4\]
/// ### 提示：

/// ### 0 <= len(arr) <= 100000
/// ### 0 <= k <= min(100000, len(arr))

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://#leetcode-cn.com/problems/smallest-k-lcci
/// ### 排序取最小
pub fn smallest_k(arr: Vec<i32>, k: usize) -> Vec<i32> {
    assert!(arr.len() >= k);
    // 排序
    let mut tem = arr.clone();
    tem.sort();
    tem.get(0..k).unwrap().to_vec()
}

use std::collections::BinaryHeap;
pub fn smallest_k1(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    assert!(arr.len() >= k);
    let mut heap = BinaryHeap::new();
    for i in 0..k {
        heap.push(arr[i])
    }
    for j in k..arr.len() {
        if let Some(max) = heap.peek() {
            if max > &arr[j] {
                heap.pop();
                heap.push(arr[j])
            }
        }
    }
    heap.into_vec()
}

#[test]
fn test_smallest_k() {
    let result = smallest_k(vec![1, 2, 3, 5, 3, -1], 4);
    println!("{:?}", result);
    let result = smallest_k1(vec![1, 3, 5, 7, 2, 4, 6, 8], 4);
    println!("{:?}", result);
}
