/// ### 有效的括号

/// ### 简单

/// ### 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。

/// ### 有效字符串需满足：

/// ### 左括号必须用相同类型的右括号闭合。
/// ### 左括号必须以正确的顺序闭合。
/// ###

/// ### 示例 1：

/// ### 输入：s = "()"
/// ### 输出：true
/// ### 示例 2：

/// ### 输入：s = "()[]{}"
/// ### 输出：true
/// ### 示例 3：

/// ### 输入：s = "(]"
/// ### 输出：false
/// ### 示例 4：

/// ### 输入：s = "([)]"
/// ### 输出：false
/// ### 示例 5：

/// ### 输入：s = "{[]}"
/// ### 输出：true
/// ###

/// ### 提示：

/// ### 1 <= s.length <= 104
/// ### s 仅由括号 '()[]{}' 组成

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/valid-parentheses
use std::collections::{HashMap, VecDeque};
pub fn is_valid(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }
    let mut v = VecDeque::new();
    let left = vec!['{', '[', '('];
    let mut map = HashMap::new();
    map.insert('}', '{');
    map.insert(']', '[');
    map.insert(')', '(');
    for c in s.chars() {
        if left.contains(&c) {
            v.push_front(c);
        } else {
            if v.is_empty() {
                return false;
            };
            let r = map.get(&c).unwrap();
            if *r == v[0] {
                v.pop_front();
            } else {
                return false;
            }
        }
    }
    if v.len() == 0 {
        return true;
    } else {
        return false;
    }
}

#[test]
fn test_is_valid() {
    let result = is_valid(String::from("{[]}"));
    println!("result :{}", result);
}
