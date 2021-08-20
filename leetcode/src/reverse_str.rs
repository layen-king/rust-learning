// 反转字符串 II
// 简单

// 给定一个字符串 s 和一个整数 k，从字符串开头算起，每 2k 个字符反转前 k 个字符。

// 如果剩余字符少于 k 个，则将剩余字符全部反转。
// 如果剩余字符小于 2k 但大于或等于 k 个，则反转前 k 个字符，其余字符保持原样。
//

// 示例 1：

// 输入：s = "abcdefg", k = 2
// 输出："bacdfeg"
// 示例 2：

// 输入：s = "abcd", k = 2
// 输出："bacd"
//

// 提示：

// 1 <= s.length <= 104
// s 仅由小写英文组成
// 1 <= k <= 104

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/reverse-string-ii
pub fn reverse_str(s: String, k: i32) -> String {
    let mut rec = true;
    let mut index = 0;
    let mut result = String::new();
    loop {
        if index > s.len() {
            break;
        }
        let count = s.len().min(index + k as usize) - index;
        let str = s.chars().skip(index).take(count).map(|ch|ch.to_string()).collect::<String>();
        if rec {
            let rev_str = &str.chars().rev().map(|ch| ch.to_string()).collect::<String>()[..];
            result.push_str(rev_str);
            rec = false;
        } else {
            result.push_str(&str[..]);
            rec = true;
        }
        index += k as usize;
    }
    result
}

#[test]
fn test_reverse_str() {
    let result = reverse_str(String::from("abcdefg"), 2);
    println!("{}", result)
}
