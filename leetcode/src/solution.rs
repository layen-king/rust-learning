use std::collections::HashMap;
/// ## 随机数索引

/// ### 中等

/// ### 给定一个可能含有重复元素的整数数组，要求随机输出给定的数字的索引。 您可以假设给定的数字一定存在于数组中。

/// ### 注意：
/// ### 数组大小可能非常大。 使用太多额外空间的解决方案将不会通过测试。

/// ### 示例:

/// ### int[] nums = new int[] {1,2,3,3,3};
/// ### Solution solution = new Solution(nums);

/// ### /// ### pick(3) 应该返回索引 2,3 或者 4。每个索引的返回概率应该相等。
/// ### solution.pick(3);

/// ### /// ### pick(1) 应该返回 0。因为只有nums[0]等于1。
/// ### solution.pick(1);
/// ### 通过次数22,485提交次数32,481
/// ### 请问您在哪类招聘中遇到此题？

/// ### 社招

/// ### 校招

/// ### 实习

/// ### 未遇到

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/random-pick-index
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
#[derive(Debug)]
pub struct Solution {
    pub map: HashMap<i32, Vec<usize>>,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let values = map.entry(nums[i]).or_insert(vec![]);
            values.push(i);
        }
        Solution { map }
    }

    pub fn pick(&self, target: i32) -> i32 {
        if let Some(nums) = self.map.get(&target) {
            let index = rand::random::<usize>() % nums.len();
            return nums[index] as i32;
        }
        0
    }
}

#[test]
fn test_solution() {
    let solution = Solution::new(vec![1, 2, 3, 3, 3]);
    assert_eq!(solution.pick(1), 0);
    assert_eq!(solution.pick(2), 1);
}
