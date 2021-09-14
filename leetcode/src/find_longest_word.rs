// 通过删除字母匹配到字典里最长单词

// 中等

// 给你一个字符串 s 和一个字符串数组 dictionary 作为字典，找出并返回字典中最长的字符串，该字符串可以通过删除 s 中的某些字符得到。

// 如果答案不止一个，返回长度最长且字典序最小的字符串。如果答案不存在，则返回空字符串。

//

// 示例 1：

// 输入：s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
// 输出："apple"
// 示例 2：

// 输入：s = "abpcplea", dictionary = ["a","b","c"]
// 输出："a"
//

// 提示：

// 1 <= s.length <= 1000
// 1 <= dictionary.length <= 1000
// 1 <= dictionary[i].length <= 1000
// s 和 dictionary[i] 仅由小写英文字母组成

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/longest-word-in-dictionary-through-deleting
pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
  // 存在bug, 某一示例下可能超时
    let mut ans:Vec<String> = vec![];
    for s2 in dictionary {
        if helper(&s, &s2) {
            if ans.is_empty() || ans[0].len() == s2.len() {
                ans.push(s2.to_owned());
            }
            if ans[0].len() < s2.len() {
                ans.clear();
                ans.push(s2.to_owned());
            }
        }
    }
    if ans.is_empty() {
        String::from("")
    } else {
        ans.sort(); 
        return ans[0].to_owned();
    }
}

fn helper(s1: &String, s2: &String) -> bool {
    let (mut i1, mut i2, mut sum) = (0, 0, 0);
    while i1 < s1.len() && i2 < s2.len() {
        if s1.chars().nth(i1) == s2.chars().nth(i2) {
            i2 += 1;
            sum += 1;
        }
        i1 += 1;
    }
    if sum == s2.len() {
        true
    } else {
        false
    }
}

#[test]
fn test_find_longest_word() {
    let c = ["ale", "apple", "monkey", "plea"].iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let result = find_longest_word(String::from("abpcplea"), c);
    assert_eq!(result, String::from("apple"))
}
