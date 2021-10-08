use std::collections::HashSet;
/// ### 重复的DNA序列

/// ### 中等

/// ### 所有 DNA 都由一系列缩写为 'A'，'C'，'G' 和 'T' 的核苷酸组成，例如："ACGAATTCCG"。在研究 DNA 时，识别 DNA 中的重复序列有时会对研究非常有帮助。

/// ### 编写一个函数来找出所有目标子串，目标子串的长度为 10，且在 DNA 字符串 s 中出现次数超过一次。

/// ###

/// ### 示例 1：

/// ### 输入：s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
/// ### 输出：["AAAAACCCCC","CCCCCAAAAA"]
/// ### 示例 2：

/// ### 输入：s = "AAAAAAAAAAAAA"
/// ### 输出：["AAAAAAAAAA"]
/// ###

/// ### 提示：

/// ### 0 <= s.length <= 105
/// ### s[i] 为 'A'、'C'、'G' 或 'T'
/// ### 通过次数46,188提交次数93,980

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/repeated-dna-sequences
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() <= 10 {
        return vec![];
    }
    let mut map = HashSet::new();
    let mut res = HashSet::new();
    for i in 0..=s.len() - 10 {
        let tem = s.clone().get(i..i + 10).unwrap().to_string();
        if map.get(&tem).is_some() {
            res.insert(tem.to_owned());
        }
        map.insert(tem.to_owned());
    }
    res.into_iter().collect::<Vec<String>>()
}

pub fn find_repeated_dna_sequences1(s: String) -> Vec<String> {
    use std::collections::HashMap;
    let s: Vec<char> = s.chars().collect();
    let mut map = HashMap::new();
    for tem in s.windows(10) {
        *map.entry(tem).or_insert(0) += 1;
    }
    map.into_iter()
        .filter(|&(_, count)| count > 1)
        .map(|(s, _)| s.iter().collect::<String>())
        .collect()
}

#[test]
fn test_find_repeated_dna_sequences() {
    let res = find_repeated_dna_sequences(String::from("AAAAAAAAAAAAA"));
    assert_eq!(res, vec![String::from("AAAAAAAAAA")]);
    let res = find_repeated_dna_sequences1(String::from("AAAAAAAAAAAAA"));
    assert_eq!(res, vec![String::from("AAAAAAAAAA")]);
}
