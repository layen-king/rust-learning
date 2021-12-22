/// ### 重复叠加字符串匹配

/// ### 中等

/// ### 给定两个字符串 a 和 b，寻找重复叠加字符串 a 的最小次数，使得字符串 b 成为叠加后的字符串 a 的子串，如果不存在则返回 -1。

/// ### 注意：字符串 "abc" 重复叠加 0 次是 ""，重复叠加 1 次是 "abc"，重复叠加 2 次是 "abcabc"。

/// ###

/// ### 示例 1：

/// ### 输入：a = "abcd", b = "cdabcdab"
/// ### 输出：3
/// ### 解释：a 重复叠加三遍后为 "abcdabcdabcd", 此时 b 是其子串。
/// ### 示例 2：

/// ### 输入：a = "a", b = "aa"
/// ### 输出：2
/// ### 示例 3：

/// ### 输入：a = "a", b = "a"
/// ### 输出：1
/// ### 示例 4：

/// ### 输入：a = "abc", b = "wxyz"
/// ### 输出：-1
/// ###

/// ### 提示：

/// ### 1 <= a.length <= 104
/// ### 1 <= b.length <= 104
/// ### a 和 b 由小写英文字母组成
/// ### 通过次数22,007提交次数59,024

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/repeated-string-match
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn repeated_string_match(a: String, b: String) -> i32 {
    let mut count = b.len() / a.len() as usize;
    if b.len() % a.len() != 0 {
      count += 1;
    }
    for i in count..=count + 1 {
        let tem = a.repeat(i);
        if tem.contains(&b) {
            return i as i32;
        }
    }
    -1
}

#[test]
fn test_repeated_string_match() {
    let result = repeated_string_match(String::from("abcd"), String::from("cdabcdab"));
    assert_eq!(result, 3)
}
