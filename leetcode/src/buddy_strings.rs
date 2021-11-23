/// ## 亲密字符串

/// ### 简单

/// ### 给你两个字符串 s 和 goal ，只要我们可以通过交换 s 中的两个字母得到与 goal 相等的结果，就返回 true ；否则返回 false 。

/// ### 交换字母的定义是：取两个下标 i 和 j （下标从 0 开始）且满足 i != j ，接着交换 s[i] 和 s[j] 处的字符。

/// ### 例如，在 "abcd" 中交换下标 0 和下标 2 的元素可以生成 "cbad" 。
/// ###

/// ### 示例 1：

/// ### 输入：s = "ab", goal = "ba"
/// ### 输出：true
/// ### 解释：你可以交换 s[0] = 'a' 和 s[1] = 'b' 生成 "ba"，此时 s 和 goal 相等。
/// ### 示例 2：

/// ### 输入：s = "ab", goal = "ab"
/// ### 输出：false
/// ### 解释：你只能交换 s[0] = 'a' 和 s[1] = 'b' 生成 "ba"，此时 s 和 goal 不相等。
/// ### 示例 3：

/// ### 输入：s = "aa", goal = "aa"
/// ### 输出：true
/// ### 解释：你可以交换 s[0] = 'a' 和 s[1] = 'a' 生成 "aa"，此时 s 和 goal 相等。
/// ### 示例 4：

/// ### 输入：s = "aaaaaaabc", goal = "aaaaaaacb"
/// ### 输出：true
/// ###

/// ### 提示：

/// ### 1 <= s.length, goal.length <= 2 * 104
/// ### s 和 goal 由小写英文字母组成
/// ### 通过次数32,196提交次数103,047

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/buddy-strings
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    if s == goal {
        let mut set = std::collections::HashSet::new();
        s.chars().for_each(|c| {
            set.insert(c);
        });
        return set.len() != goal.len();
    } else {
        let (mut first, mut second) = (usize::MAX, usize::MAX);
        let (s, goal) = (s.as_bytes(), goal.as_bytes());
        for i in 0..s.len() {
            if s[i] != goal[i] {
                if first == usize::MAX {
                    first = i;
                } else if second == usize::MAX {
                    second = i;
                } else {
                    return false;
                }
            }
        }
        if first != usize::MAX
            && second != usize::MAX
            && s[first] == goal[second]
            && s[second] == goal[first]
        {
            return true;
        }
    }
    false
}

#[test]
fn test_buddy_strings() {
    let result = buddy_strings(String::from("aa"), String::from("aa"));
    assert_eq!(result, false);
    let result = buddy_strings(String::from("ab"), String::from("ba"));
    assert_eq!(result, true);
    let result = buddy_strings(String::from("aaaaabc"), String::from("aaaaacb"));
    assert_eq!(result, true);
}
