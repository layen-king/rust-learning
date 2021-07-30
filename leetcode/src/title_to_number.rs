/// ## Excel 表列序号

/// ### 给你一个字符串 columnTitle ，表示 Excel 表格中的列名称。返回该列名称对应的列序号。

/// ### 例如，

/// ###     A -> 1
/// ###     B -> 2
/// ###     C -> 3
/// ###     ...
/// ###     Z -> 26
/// ###     AA -> 27
/// ###     AB -> 28
/// ###     ...
/// ###

/// ### 示例 1:

/// ### 输入: columnTitle = "A"
/// ### 输出: 1
/// ### 示例 2:

/// ### 输入: columnTitle = "AB"
/// ### 输出: 28
/// ### 示例 3:

/// ### 输入: columnTitle = "ZY"
/// ### 输出: 701
/// ### 示例 4:

/// ### 输入: columnTitle = "FXSHRXW"
/// ### 输出: 2147483647
pub fn title_to_number(column_title: String) -> i32 {
    let mut res = 0;
    let mut current = 1;
    let mut vec = column_title
        .chars()
        .map(|c| return c as u32)
        .collect::<Vec<u32>>();
    println!("vec: {:?}", vec);
    vec.reverse();
    for i in vec.iter() {
        let k = *i - 'A' as u32 + 1;
        res += k * current;
        current *= 26;
    }
    res as i32
}

#[test]
fn test_title_to_number() {
    let s = String::from("AB");
    assert_eq!(title_to_number(s), 28);
}
