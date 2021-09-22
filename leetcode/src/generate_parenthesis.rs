/// ## 括号生成

/// ### 中等

/// ### 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

/// ### 有效括号组合需满足：左括号必须以正确的顺序闭合。

/// ###

/// ### 示例 1：

/// ### 输入：n = 3
/// ### 输出：["((()))","(()())","(())()","()(())","()()()"]
/// ### 示例 2：

/// ### 输入：n = 1
/// ### 输出：["()"]
/// ###

/// ### 提示：

/// ### 1 <= n <= 8
/// ### 通过次数348,060提交次数450,572

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/generate-parentheses
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    if n <= 0 {
        return res;
    };
    fn dfs(path: &mut String, left: i32, right: i32, n: i32, vec: &mut Vec<String>) {
        if left > n || right > left {
            return;
        };
        if (path.len() as i32) == n * 2 {
            vec.push(path.to_owned());
        }
        let mut left_str = format!("{}(", path);
        let mut right_str = format!("{})", path);
        dfs(&mut left_str, left + 1, right, n, vec);
        dfs(&mut right_str, left, right + 1, n, vec);
    }
    dfs(&mut String::from(""), 0, 0, n, &mut res);
    res
}

#[test]
fn test_generate_parentheses() {
    let result = generate_parenthesis(1);
    println!("result:{:?}", result);
    assert_eq!(result, vec![String::from("()")])
}
