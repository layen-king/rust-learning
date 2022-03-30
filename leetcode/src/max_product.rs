/// ## 最大单词长度乘积

/// ### 中等

/// ### 给定一个字符串数组 words，找到 length(word\[i\]) * length(word\[j\]) 的最大值，
/// 并且这两个单词不含有公共字母。你可以认为每个单词只包含小写字母。如果不存在这样的两个单词，返回 0。

/// ###

/// ### 示例 1:

/// ### 输入: \["abcw","baz","foo","bar","xtfn","abcdef"\]
/// ### 输出: 16
/// ### 解释: 这两个单词为 "abcw", "xtfn"。
/// ### 示例 2:

/// ### 输入: \["a","ab","abc","d","cd","bcd","abcd"\]
/// ### 输出: 4
/// ### 解释: 这两个单词为 "ab", "cd"。
/// ### 示例 3:

/// ### 输入: \["a","aa","aaa","aaaa"\]
/// ### 输出: 0
/// ### 解释: 不存在这样的两个单词。
/// ###

/// ### 提示：

/// ### 2 <= words.length <= 1000
/// ### 1 <= words\[i\].length <= 1000
/// ### words\[i\] 仅包含小写字母
/// ### 通过次数29,262提交次数40,833

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/maximum-product-of-word-lengths
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn max_product(words: Vec<String>) -> i32 {
    let mut words = words.clone();
    let mut res = 0i32;
    words.sort_by(|x, y| y.len().cmp(&x.len()));
    for i in 0..words.len() {
        for j in i + 1..words.len() {
            let tem: i32 = (words[j].len() as i32) * (words[i].len() as i32);
            if tem > res && help(&words[i], &words[j]) {
                res = tem
            }
        }
    }
    res
}

fn help(str1: &str, str2: &str) -> bool {
    for i in str2.chars() {
        if str1.find(i).is_some() {
            return false;
        }
    }
    true
}

#[test]
fn test_max_product() {
    let words: Vec<String> = [
        "eae", "ea", "aaf", "bda", "fcf", "dc", "ac", "ce", "cefde", "dabae",
    ]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>();
    let result = max_product(words);
    println!("{}", result);
}
