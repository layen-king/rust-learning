// 盛最多水的容器

// 给你 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0) 。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

// 说明：你不能倾斜容器。

// 输入：\[1,8,6,2,5,4,8,3,7\]
// 输出：49
// 解释：图中垂直线代表输入数组 \[1,8,6,2,5,4,8,3,7\]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。

// 示例 2：

// 输入：height = \[1,1\]
// 输出：1

// 示例 3：

// 输入：height = \[4,3,2,1,4\]
// 输出：16
// 示例 4：

// 输入：height = \[1,2,1\]
// 输出：2
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut res = 0;
    // 双指针
    let mut left = 0;
    let mut right = height.len() - 1;
    while left < right {
        // 取最小高度乘以间距
        let sum = height[left].min(height[right]) * (right - left) as i32;
        // 取最大面积
        res = res.max(sum);
        // 移动高度更低的指针
        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }
    }
    res
}

#[test]
fn test_max_area() {
    let v1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let v2 = vec![1, 1];
    let v3 = vec![4, 3, 2, 1, 4];
    let v4 = vec![1, 2, 1];
    assert_eq!(max_area(v1), 49);
    assert_eq!(max_area(v2), 1);
    assert_eq!(max_area(v3), 16);
    assert_eq!(max_area(v4), 2)
}
