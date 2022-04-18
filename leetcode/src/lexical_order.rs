/// ### 字典序排数

/// ### 中等

/// ### 给你一个整数 n ，按字典序返回范围 [1, n] 内所有整数。

/// ### 你必须设计一个时间复杂度为 O(n) 且使用 O(1) 额外空间的算法。

/// ###

/// ### 示例 1：

/// ### 输入：n = 13
/// ### 输出：[1,10,11,12,13,2,3,4,5,6,7,8,9]
/// ### 示例 2：

/// ### 输入：n = 2
/// ### 输出：[1,2]
/// ###

/// ### 提示：

/// ### 1 <= n <= 5 * 104
/// ### 通过次数34,218提交次数45,006

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/lexicographical-numbers
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut cur = 1i32;
    for _ in 0..n {
        res.push(cur);
        if cur * 10 <= n {
            cur = cur * 10;
        } else {
            while cur > n - 1 || cur % 10 == 9 {
                cur = cur / 10;
            }
            cur = cur + 1;
        }
    }
    res
}

#[test]
fn test_lexical_order() {
    let res = lexical_order(13);
    assert_eq!(res, vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9])
}
