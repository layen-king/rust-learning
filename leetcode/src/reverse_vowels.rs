/// ## 反转字符串中的元音字母
/// ### 简单

/// ### 编写一个函数，以字符串作为输入，反转该字符串中的元音字母。

/// ###

/// ### 示例 1：

/// ### 输入："hello"
/// ### 输出："holle"
/// ### 示例 2：

/// ### 输入："leetcode"
/// ### 输出："leotcede"
/// ###

/// ### 提示：

/// ### 元音字母不包含字母 "y" 。

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/reverse-vowels-of-a-string
pub fn reverse_vowels(s: String) -> String {
    let mut chars = s.clone().chars().collect::<Vec<char>>();
    let (mut left, mut right) = (0, chars.len() - 1);
    while left < right {
        while left < right && !is_vowel(chars[left]) {
            left += 1;
        }
        while left < right && !is_vowel(chars[right]) {
            right -= 1;
        }
        if left < right {
            let tem = chars[left].clone();
            chars[left] = chars[right];
            chars[right] = tem;
            left += 1;
            right -= 1;
        }
    }
    chars.into_iter().collect::<String>()
}

fn is_vowel(c: char) -> bool {
    return "aeiouAEIOU".contains(&c.to_string()[..]);
}

use std::collections::HashSet;
pub fn reverse_vowels_1(s: String) -> String {
    let hs: HashSet<char> = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
        .iter()
        .cloned()
        .collect();
    let mut cv = s.chars().collect::<Vec<char>>();
    let (mut left, mut right) = (0, cv.len() - 1);
    while left < right {
        if !hs.contains(&cv[left]) {
            left += 1;
        } else if !hs.contains(&cv[right]) {
            right -= 1;
        } else {
            cv.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    cv.into_iter().collect::<String>()
}

#[test]
fn test_reverse_vowels() {
    let result = reverse_vowels(String::from("hello"));
    assert_eq!(result, String::from("holle"));
    let result = reverse_vowels(String::from("leetcode"));
    assert_eq!(result, String::from("leotcede"));
}
