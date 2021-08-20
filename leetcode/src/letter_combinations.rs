use std::collections::HashMap;
/// ## 电话号码的字母组合
/// ### 中等

/// ### 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。

/// ### 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母
/// ### 示例 1：

/// ### 输入：digits = "23"
/// ### 输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
/// ### 示例 2：

/// ### 输入：digits = ""
/// ### 输出：[]
/// ### 示例 3：

/// ### 输入：digits = "2"
/// ### 输出：["a","b","c"]
/// ###

/// ### 提示：

/// ### 0 <= digits.length <= 4
/// ### digits[i] 是范围 ['2', '9'] 的一个数字。

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return Vec::<String>::new();
    }
    let mut map = HashMap::new();
    map.insert("2", "abc");
    map.insert("3", "def");
    map.insert("4", "ghi");
    map.insert("5", "jkl");
    map.insert("6", "mno");
    map.insert("7", "pqrs");
    map.insert("8", "tuv");
    map.insert("9", "wxyz");

    let mut result: Vec<String> = vec![];
    dfs("", 0, &digits, &mut result, &map);
    result
}

fn dfs(str: &str, i: usize, digits: &String, result: &mut Vec<String>, map: &HashMap<&str, &str>) {
    if i > digits.len() - 1 {
        result.push(str.to_owned());
    } else {
        let k = &digits.chars().nth(i).unwrap().to_string()[..];
        let leeters = map.get(&k).unwrap();
        for c in leeters.chars() {
          let s = str.to_string() + &c.to_string()[..];
            dfs(&s[..], i + 1, digits, result, map);
        }
    }
}

#[test]
fn test_letter_combinations() {
    let result = letter_combinations(String::from("23"));
    println!("{:?}", result);
}
