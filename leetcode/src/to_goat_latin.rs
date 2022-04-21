/// ## 山羊拉丁文

/// ### 简单

/// ### 给你一个由若干单词组成的&&&&句子 sentence ，单词间由空格分隔。每个单词仅由大写和小写英文字母组成。

/// ### 请你将句子转换为 “山羊拉丁文（Goat Latin）”（一种类似于 猪拉丁文 - Pig Latin 的虚构语言）。山羊拉丁文的规则如下：

/// ### 如果单词以元音开头（'a', 'e', 'i', 'o', 'u'），在单词后添加"ma"。
/// ### 例如，单词 "apple" 变为 "applema" 。
/// ### 如果单词以辅音字母开头（即，非元音字母），移除第一个字符并将它放到末尾，之后再添加"ma"。
/// ### 例如，单词 "goat" 变为 "oatgma" 。
/// ### 根据单词在句子中的索引，在单词最后添加与索引相同数量的字母'a'，索引从 1 开始。
/// ### 例如，在第一个单词后添加 "a" ，在第二个单词后添加 "aa" ，以此类推。
/// ### 返回将 sentence 转换为山羊拉丁文后的句子。

/// ###

/// ### 示例 1：

/// ### 输入：sentence = "I speak Goat Latin"
/// ### 输出："Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
/// ### 示例 2：

/// ### 输入：sentence = "The quick brown fox jumped over the lazy dog"
/// ### 输出："heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
/// ###

/// ### 提示：

/// ### 1 <= sentence.length <= 150
/// ### sentence 由英文字母和空格组成
/// ### sentence 不含前导或尾随空格
/// ### sentence 中的所有单词由单个空格分隔
/// ### 通过次数31,046提交次数47,982

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/goat-latin
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn to_goat_latin(sentence: String) -> String {
    let mut words = sentence.split_whitespace();
    let mut res = String::new();
    let mut a = String::from("a");
    let owels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    loop {
        if let Some(word) = words.next() {
            let c = word.as_bytes()[0] as char;
            if !owels.contains(&c) {
                let mut tem = String::from(word);
                tem.remove(0);
                tem.push(c);
                res.push_str(&tem);
            } else {
                res.push_str(word);
            }
            res.push_str(format!("ma{} ", &a.clone()).as_str());
            a.push('a');
        } else {
            break;
        }
    }
    res.trim().to_string()
}

#[test]
fn test_to_goat_latin() {
    let result = to_goat_latin(String::from("I speak Goat Latin"));
    assert_eq!(result, String::from("Imaa peaksmaaa oatGmaaaa atinLmaaaaa"))
}
