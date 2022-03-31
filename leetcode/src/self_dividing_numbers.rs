/// ## 自除数

/// ### 简单

/// ### 自除数 是指可以被它包含的每一位数整除的数。

/// ### 例如，128 是一个 自除数 ，因为 128 % 1 == 0，128 % 2 == 0，128 % 8 == 0。
/// ### 自除数 不允许包含 0 。

/// ### 给定两个整数 left 和 right ，返回一个列表，列表的元素是范围 \[left, right\] 内所有的 自除数 。

/// ###

/// ### 示例 1：

/// ### 输入：left = 1, right = 22
/// ### 输出：\[1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22\]
/// ### 示例 2:

/// ### 输入：left = 47, right = 85
/// ### 输出：\[48,55,66,77\]
/// ###

/// ### 提示：

/// ### 1 <= left <= right <= 104
/// ### 通过次数56,927提交次数72,651

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://eetcode-cn.com/problems/self-dividing-numbers
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right).filter(|x| help(x)).collect::<Vec<_>>()
}

fn help(i: &i32) -> bool {
    let mut tem = i.clone();
    while tem > 0 {
        let k = tem % 10;
        if k == 0 || i % k != 0 {
            return false;
        }
        tem = tem / 10;
    }
    true
}

#[test]
fn test_self_dividing_numbers() {
    let result = self_dividing_numbers(1, 22);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22], result);
}
