use std::collections::{HashSet, VecDeque};

/// ## 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。

/// ### 示例 1:

/// ### 输入: s = "abcabcbb"
/// ### 输出: 3
/// ### 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
/// ### 示例 2:

/// ### 输入: s = "bbbbb"
/// ### 输出: 1
/// ### 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
/// ### 示例 3:

/// ### 输入: s = "pwwkew"
/// ### 输出: 3
/// ### 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
/// ###      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
/// ### 示例 4:

/// ### 输入: s = ""
/// ### 输出: 0

/// ### 提示：

/// ### 0 <= s.length <= 5 * 104
/// ### s 由英文字母、数字、符号和空格组成
#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.into_bytes();
    let mut max = 0;
    let mut q = VecDeque::new();
    let mut set = HashSet::new();
    for i in 0..s.len() {
        if set.contains(&s[i]) {
            while let Some(x) = q.pop_front() {
                set.remove(&x);
                if x == s[i] {
                    break;
                }
            }
        }
        set.insert(s[i]);
        q.push_back(s[i]);
        max = max.max(q.len());
    }
    max as i32
}
#[allow(dead_code)]
pub fn length_of_longest_substring1(s: String) -> i32 {
    let s = s.into_bytes();
    let mut max = 0;
    let mut set = HashSet::new();
    let len = s.len();
    for i in 0..len {
        for j in i + set.len()..len {
            if set.contains(&s[j]) {
                break;
            }
            set.insert(&s[j]);
        }
        max = max.max(set.len());
        if max >= (len - i) {
            break;
        }
        set.clear();
    }
    max as i32
}
