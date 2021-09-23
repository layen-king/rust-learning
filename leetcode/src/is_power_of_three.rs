// 3的幂

// 简单

// 给定一个整数，写一个函数来判断它是否是 3 的幂次方。如果是，返回 true ；否则，返回 false 。

// 整数 n 是 3 的幂次方需满足：存在整数 x 使得 n == 3x

//

// 示例 1：

// 输入：n = 27
// 输出：true
// 示例 2：

// 输入：n = 0
// 输出：false
// 示例 3：

// 输入：n = 9
// 输出：true
// 示例 4：

// 输入：n = 45
// 输出：false
//

// 提示：

// -231 <= n <= 231 - 1
//

// 进阶：

// 你能不使用循环或者递归来完成本题吗？

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/power-of-three
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn is_power_of_three(n: i32) -> bool {
    let mut n = n as f64;
    while n >= 3.0 {
        n = n / 3.0;
        if n % 1.0 != 0.0 {
            return false;
        }
    }
    if n == 1.0 {
        true
    } else {
        false
    }
}

pub fn is_power_of_three1(n: i32) -> bool {
    if n <= 0 {return false;}
    let max = 3_i32.pow(19);
    if max % n == 0 {
        true
    } else {
        false
    }
}

#[test]
fn test_is_power_of_three() {
    let result = is_power_of_three(45);
    assert_eq!(result, false);
    let result = is_power_of_three1(45);
    assert_eq!(result, false)
}
