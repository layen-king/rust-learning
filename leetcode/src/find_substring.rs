use std::collections::HashMap;
/// ## 串联所有单词的子串

/// ### 困难

/// ### 给定一个字符串 s 和一些 长度相同 的单词 words 。找出 s 中恰好可以由 words 中所有单词串联形成的子串的起始位置。

/// ### 注意子串要与 words 中的单词完全匹配，中间不能有其他字符 ，但不需要考虑 words 中单词串联的顺序。

/// ###

/// ### 示例 1：

/// ### 输入：s = "barfoothefoobarman", words = ["foo","bar"]
/// ### 输出：[0,9]
/// ### 解释：
/// ### 从索引 0 和 9 开始的子串分别是 "barfoo" 和 "foobar" 。
/// ### 输出的顺序不重要, [9,0] 也是有效答案。
/// ### 示例 2：

/// ### 输入：s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
/// ### 输出：[]
/// ### 示例 3：

/// ### 输入：s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
/// ### 输出：[6,9,12]
/// ###

/// ### 提示：

/// ### 1 <= s.length <= 104
/// ### s 由小写英文字母组成
/// ### 1 <= words.length <= 5000
/// ### 1 <= words[i].length <= 30
/// ### words[i] 由小写英文字母组成
/// ### 通过次数104,021提交次数281,492

/// ### 来源：力扣（LeetCode）
/// ### 链接：https:/// ###leetcode-cn.com/problems/substring-with-concatenation-of-all-words
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut words_map = HashMap::new();
    for i in 0..words.len() {
        let counter = words_map.entry(&words[i]).or_insert(0);
        *counter += 1;
    }
    let (word_len, mut res) = (words[0].len(), vec![]);
    let words_len = word_len * words.len();
    for i in 0..s.len() {
        let mut tem_map = HashMap::new();
        let mut j = i;
        while j < i + words_len {
            if let Some(s) = s.get(j..j + word_len) {
                if !words.contains(&s.to_owned()) {
                    break;
                } else {
                    let counter = tem_map.entry(s).or_insert(0);
                    *counter += 1;
                    j = j + word_len;
                }
            } else {
                break;
            }
        }
        if words_map.len() == tem_map.len() {
            let mut flag = true;
            for (key, value) in words_map.iter() {
                if let Some(v) = tem_map.get(key.as_str()) {
                    if v != value {
                        flag = false;
                        break;
                    }
                }
            }
            if flag {
                res.push(i as i32);
            }
        }
    }
    res
}
