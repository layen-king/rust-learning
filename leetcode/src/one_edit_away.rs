/// ## 面试题 01.05. 一次编辑

/// ### 中等

/// ### 字符串有三种编辑操作:插入一个字符、删除一个字符或者替换一个字符。 给定两个字符串，编写一个函数判定它们是否只需要一次(或者零次)编辑。

/// ###

/// ### 示例 1:

/// ### 输入:
/// ### first = "pale"
/// ### second = "ple"
/// ### 输出: True
/// ###

/// ### 示例 2:

/// ### 输入:
/// ### first = "pales"
/// ### second = "pal"
/// ### 输出: False
/// ### 通过次数58,834提交次数170,316

/// ### 来源：力扣（LeetCode）
/// ### 链接：https:/// ###leetcode.cn/problems/one-away-lcci
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn one_edit_away(first: String, second: String) -> bool {
    let (first, second) = if first.len() > second.len() {
        (first.as_bytes(), second.as_bytes())
    } else {
        (second.as_bytes(), first.as_bytes())
    };
    let (m, n) = (first.len(), second.len());
    if m - n > 1 {
        return false;
    }
    let (mut i, mut j, mut cnt) = (0, 0, 0);
    while i < m {
        if first.get(i) != second.get(j) {
            cnt += 1;
            if m != n {
                j -= 1;
            }
        }
        if cnt > 1 {
            return false;
        }
        i += 1;
        j += 1;
    }
    true
}

#[test]
fn test_one_edit_away() {
    let result = one_edit_away(String::from("pales"), String::from("pal"));
    assert_eq!(result, false)
}
