/// ## 加一

/// ### 简单

/// ### 给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一。

/// ### 最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。

/// ### 你可以假设除了整数 0 之外，这个整数不会以零开头。

/// ###

/// ### 示例 1：

/// ### 输入：digits = [1,2,3]
/// ### 输出：[1,2,4]
/// ### 解释：输入数组表示数字 123。
/// ### 示例 2：

/// ### 输入：digits = [4,3,2,1]
/// ### 输出：[4,3,2,2]
/// ### 解释：输入数组表示数字 4321。
/// ### 示例 3：

/// ### 输入：digits = [0]
/// ### 输出：[1]
/// ###

/// ### 提示：

/// ### 1 <= digits.length <= 100
/// ### 0 <= digits[i] <= 9
/// ### 通过次数371,240提交次数804,837

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/plus-one
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn one_plus(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits.clone();
    for i in digits.len() - 1..=0 {
        if digits[i] != 9 {
            digits[i] = digits[i] + 1;
            return digits;
        } else {
            digits[i] = 0;
        }
    }
    if digits[0] == 0 {
        digits.insert(0, 1);
    }
    digits
}

#[test]
fn test_one_plus() {
    let result = one_plus(vec![1, 2, 3, 4]);
    assert_eq!(result, vec![1, 2, 3, 5])
}
