/// ## 三数之和
/// ### 中等

/// ### 给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有和为 0 且不重复的三元组。

/// ### 注意：答案中不可以包含重复的三元组。

/// ###

/// ### 示例 1：

/// ### 输入：nums = \[-1,0,1,2,-1,-4\]
/// ### 输出：[[-1,-1,2],[-1,0,1]]
/// ### 示例 2：

/// ### 输入：nums = []
/// ### 输出：[]
/// ### 示例 3：

/// ### 输入：nums = \[0\]
/// ### 输出：[]
/// ###

/// ### 提示：

/// ### 0 <= nums.length <= 3000
/// ### -105 <= nums\[i\] <= 105

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/3sum
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    if len < 3 {
        return vec![];
    };
    nums.sort();
    let mut ans = vec![];
    let len = nums.len();
    if nums[len - 1] < 0 || nums[0] > 0 {
        return vec![];
    };
    for i in 0..len {
        if i > 0 && nums[i - 1] == nums[i] {
            continue;
        };
        let [mut head, mut tail] = [i + 1, len - 1];
        while head < tail {
            if head > i + 1 && nums[head] == nums[head - 1] {
                head += 1;
                continue;
            }
            let sum = nums[head] + nums[tail] + nums[i];
            if sum < 0 {
                head += 1;
            } else if sum > 0 {
                tail -= 1;
            } else {
                ans.push(vec![nums[i], nums[head], nums[tail]]);
                head += 1
            }
        }
    }
    ans
}

#[test]
fn test_three_sum() {
    let vec = [-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4]
        .iter()
        .map(|i| i.clone())
        .collect::<Vec<_>>();
    println!("{:?}", three_sum(vec));
}
