/// ## 最长回文子串

/// ### 给你一个字符串 s，找到 s 中最长的回文子串。

/// ### 示例 1：
/// ### 输入：s = "babad"
/// ### 输出："bab"
/// ### 解释："aba" 同样是符合题意的答案。

/// ### 示例 2：
/// ### 输入：s = "cbbd"
/// ### 输出："bb"

/// ### 示例 3：
/// ### 输入：s = "a"
/// ### 输出："a"

/// ### 示例 4：
/// ### 输入：s = "ac"
/// ### 输出："a"

/// ### 中心扩散
#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    if s.len() == 1 {
        return s;
    }
    let mut max_len_string = String::from("");
    for i in 0..s.len() {
        let mut left = if i >= 1 { i - 1 } else { 0 };
        let mut right = i + 1;
        let mut item_str = take_bytes(&s, i);
        let mut coiled = true;
        while right < s.len() {
            let right_str = take_bytes(&s, right);
            if coiled && &item_str == &right_str {
                item_str = format!("{}{}", &item_str, &right_str);
                right += 1
            } else if &take_bytes(&s, left) == &right_str {
                item_str = format!("{}{}{}", &right_str, &item_str, &right_str);
                if left == 0 {
                    break;
                } else {
                    left -= 1
                }
                right += 1;
                coiled = false;
            } else {
                break;
            }
        }
        max_len_string = if max_len_string.len() >= item_str.len() {
            max_len_string
        } else {
            item_str
        }
    }
    max_len_string
}

fn take_bytes(str: &str, n: usize) -> String {
    let chars = str.chars();
    let mut index = 0;
    for i in chars {
        if index == n {
            return String::from(i);
        }
        index += 1;
    }
    return String::from("");
}

#[test]
pub fn test_longest_palindrome() {
    // let s = String::from("abab");
    // let str = longest_palindrome(s);
    // assert_eq!(str, String::from("aba"));

    // let s1 = String::from("babadeff");
    // let str = longest_palindrome(s1);
    // assert_eq!(str, String::from("bab"));

    // let s2 = String::from("bb");
    // let str = longest_palindrome(s2);
    // assert_eq!(str, String::from("bb"));

    let s2 = String::from("ccc");
    let str = longest_palindrome(s2);
    assert_eq!(str, String::from("ccc"));
}
