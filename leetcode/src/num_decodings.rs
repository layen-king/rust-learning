/// ## 解码方法 II

/// ### 困难

/// ### 一条包含字母 A-Z 的消息通过以下的方式进行了编码：

/// ### 'A' -> 1
/// ### 'B' -> 2
/// ### ...
/// ### 'Z' -> 26
/// ### 要 解码 一条已编码的消息，所有的数字都必须分组，然后按原来的编码方案反向映射回字母（可能存在多种方式）。例如，"11106" 可以映射为：

/// ### "AAJF" 对应分组 (1 1 10 6)
/// ### "KJF" 对应分组 (11 10 6)
/// ### 注意，像 (1 11 06) 这样的分组是无效的，因为 "06" 不可以映射为 'F' ，因为 "6" 与 "06" 不同。

/// ### 除了 上面描述的数字字母映射方案，编码消息中可能包含 '*' 字符，可以表示从 '1' 到 '9' 的任一数字（不包括 '0'）。例如，编码字符串 "1*" 可以表示 "11"、"12"、"13"、"14"、"15"、"16"、"17"、"18" 或 "19" 中的任意一条消息。对 "1*" 进行解码，相当于解码该字符串可以表示的任何编码消息。

/// ### 给你一个字符串 s ，由数字和 '*' 字符组成，返回 解码 该字符串的方法 数目 。

/// ### 由于答案数目可能非常大，返回对 109 + 7 取余 的结果。

/// ###

/// ### 示例 1：

/// ### 输入：s = "*"
/// ### 输出：9
/// ### 解释：这一条编码消息可以表示 "1"、"2"、"3"、"4"、"5"、"6"、"7"、"8" 或 "9" 中的任意一条。
/// ### 可以分别解码成字符串 "A"、"B"、"C"、"D"、"E"、"F"、"G"、"H" 和 "I" 。
/// ### 因此，"*" 总共有 9 种解码方法。
/// ### 示例 2：

/// ### 输入：s = "1*"
/// ### 输出：18
/// ### 解释：这一条编码消息可以表示 "11"、"12"、"13"、"14"、"15"、"16"、"17"、"18" 或 "19" 中的任意一条。
/// ### 每种消息都可以由 2 种方法解码（例如，"11" 可以解码成 "AA" 或 "K"）。
/// ### 因此，"1*" 共有 9 * 2 = 18 种解码方法。
/// ### 示例 3：

/// ### 输入：s = "2*"
/// ### 输出：15
/// ### 解释：这一条编码消息可以表示 "21"、"22"、"23"、"24"、"25"、"26"、"27"、"28" 或 "29" 中的任意一条。
/// ### "21"、"22"、"23"、"24"、"25" 和 "26" 由 2 种解码方法，但 "27"、"28" 和 "29" 仅有 1 种解码方法。
/// ### 因此，"2*" 共有 (6 * 2) + (3 * 1) = 12 + 3 = 15 种解码方法。
/// ###

/// ### 提示：

/// ### 1 <= s.length <= 105
/// ### s[i] 是 0 - 9 中的一位数字或字符 '*'
/// ### 通过次数5,322提交次数15,832

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/decode-ways-ii
pub fn num_decodings(s: String) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let (mut a, mut b, mut res) = (0_i64, 1_i64, 0_i64);
    for i in 1..=s.len() {
        res = b * helper1(&s.chars().nth(i - 1).unwrap()) % MOD;
        if i > 1 {
            res =
                (res + a * helper2(
                    &s.chars().nth(i - 2).unwrap(),
                    &s.chars().nth(i - 1).unwrap(),
                )) % MOD;
        }
        a = b;
        b = res;
    }
    res as i32
}

fn helper1(s1: &char) -> i64 {
    if *s1 == '0' {
        return 0;
    } else {
        if *s1 == '*' {
            return 9;
        } else {
            1
        }
    }
}

fn helper2(s1: &char, s2: &char) -> i64 {
    if *s1 == '*' && *s2 == '*' {
        return 15;
    }
    if *s1 == '*' {
        if s2.to_digit(10).unwrap_or(0) <= 6 {
            return 2;
        } else {
            return 1;
        }
    }
    if *s2 == '*' {
        if *s1 == '1' {
            return 9;
        } else if *s1 == '2' {
            return 6;
        } else {
            return 0;
        }
    }
    if *s1 != '0' && s1.to_digit(10).unwrap_or(1) * 10 + s2.to_digit(10).unwrap_or(1) <= 26 {
        return 1;
    }
    0
}

pub fn num_decodings1(s: String) -> i32 {
    const M:i64 = 1000000007;
    let init = (
        1,
        match s.get(0..=0) {
            Some("*") => 9,
            Some("0") => 0,
            _ => 1,
        },
    );
    s.as_bytes()
        .windows(2)
        .fold(init, |(first, mut second), x| {
            let (last, current, temp) = (x[0], x[1], second);
            if current == b'*' {
                second = match last {
                    b'1' => (second + first) * 9 % M,
                    b'2' => (second * 9 + first * 6) % M,
                    b'*' => (second * 9 + first * 15) % M,
                    _ => second * 9,
                }
            } else {
                second *= (current != b'0') as i64;
                second = if last == b'1' {
                    (second + first) % M
                } else if last == b'2' && current <= b'6' {
                    (second + first) % M
                } else if last == b'*' {
                    (second + (1 + (current <= b'6') as i64) * first) % M
                } else {
                    second
                }
            }
            (temp, second)
        })
        .1 as i32
}

#[test]
fn test_num_decodings() {
    let result = num_decodings(String::from("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*"));
    assert_eq!(result, 196465252);
    let result = num_decodings1(String::from("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*"));
    assert_eq!(result, 196465252);
}
