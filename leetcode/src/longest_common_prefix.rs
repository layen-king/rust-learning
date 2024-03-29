/// ## 最长公共前缀
/// ### 简单

/// ### 编写一个函数来查找字符串数组中的最长公共前缀。

/// ### 如果不存在公共前缀，返回空字符串 ""。

/// ###

/// ### 示例 1：

/// ### 输入：strs = \["flower","flow","flight"\]
/// ### 输出："fl"
/// ### 示例 2：

/// ### 输入：strs = \["dog","racecar","car"\]
/// ### 输出：""
/// ### 解释：输入不存在公共前缀。
/// ###

/// ### 提示：

/// ### 1 <= strs.length <= 200
/// ### 0 <= strs\[i\].length <= 200
/// ### strs\[i\] 仅由小写英文字母组成

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/longest-common-prefix
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = vec![];
    for i in 0..strs[0].len() {
        let current_char = strs[0].chars().nth(i);
        let pass = strs
            .iter()
            .all(|f| f.chars().nth(i) == current_char);
        if pass {
            result.push(current_char.unwrap());
        } else {
            break;
        }
    }
    result.iter().collect::<String>()
}

#[test]
fn test_longest_palindrome_prefix() {
    let k = ["flower", "flow", "flight"]
        .iter()
        .map(|s| s.to_owned().to_string())
        .collect::<Vec<_>>();
    let result = longest_common_prefix(k);
    assert_eq!(result, String::from("fl"))
}
