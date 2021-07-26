#[test]
pub fn test_conver() {
    let result = convert(String::from("PAYPALISHIRING"), 3);
    println!("result: {}", result);
    assert_eq!(result, String::from("PAHNAPLSIIGYIR"));
}

/// ## Z 字形变换

/// ### 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。

/// ### 比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：

/// ### P   A   H   N
/// ### A P L S I I G
/// ### Y   I   R
/// ### 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。

/// ### 请你实现这个将字符串进行指定行数变换的函数：

/// ### string convert(string s, int numRows);
/// ###  

/// ### 示例 1：

/// ### 输入：s = "PAYPALISHIRING", numRows = 3
/// ### 输出："PAHNAPLSIIGYIR"
/// ### 示例 2：
/// ### 输入：s = "PAYPALISHIRING", numRows = 4
/// ### 输出："PINALSIGYAHRPI"
/// ### 解释：
/// ### P     I    N
/// ### A   L S  I G
/// ### Y A   H R
/// ### P     I
/// ### 示例 3：

/// ### 输入：s = "A", numRows = 1
/// ### 输出："A"
pub fn convert(s: String, num_rows: i32) -> String {
    if s.len() <= (num_rows as usize) || num_rows == 1 {
        return s;
    }
    let mut v = Vec::with_capacity(num_rows as usize);
    for _ in 0..num_rows {
        v.push(String::from(""));
    }
    println!("v length: {}", v.len());
    let mut plus = true;
    let mut num = 0;
    for i in 0..s.len() {
        let char = s.chars().nth(i).unwrap().clone().to_string();
        v[num] = char;
        if plus {
            num += 1;
        } else {
            num -= 1;
        }
        if num == 0 {
            plus = true
        }
        if num == (num_rows as usize) - 1 {
            plus = false
        }
    }
    v.into_iter().collect()
}
