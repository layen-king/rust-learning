/// ## 回文数

/// ### 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。

/// ### 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。例如，121 是回文，而 123 不是。

/// ###

/// ### 示例 1：

/// ### 输入：x = 121
/// ### 输出：true
/// ### 示例 2：

/// ### 输入：x = -121
/// ### 输出：false
/// ### 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
/// ### 示例 3：

/// ### 输入：x = 10
/// ### 输出：false
/// ### 解释：从右向左读, 为 01 。因此它不是一个回文数。
/// ### 示例 4：

/// ### 输入：x = -101
/// ### 输出：false

/// ### 使用字符串
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let str = x.to_string();
    if str.len() == 1 {
        return true;
    }
    let vec: Vec<u32> = str.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut left: usize = 0;
    let mut right = vec.len() - 1;
    while left <= right {
        if vec[left] != vec[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

/// ## 使用除法取余数
pub fn is_palindrome_1(x: i32) -> bool {
    // 1位返回true
    if x >= 0 && x <= 9 {
        return true;
    }
    // 末尾是0,和小于0 直接返回false
    if x % 10 == 0 || x < 0 {
        return false;
    }
    let mut x = x as usize;
    let mut y: usize = 0;
    while x > y {
        y = y * 10 + x % 10;
        x = x / 10;
        println!("x: {}", x);
        println!("y: {}", y);
    }
    if x != 0 && y != 0 && (x == y || y / 10 == x) {
        return true;
    }
    false
}

#[test]
fn test_is_palindrome() {
    let result = is_palindrome(0);
    assert_eq!(result, true);
    let result = is_palindrome(121);
    assert_eq!(result, true);
    let result = is_palindrome(-1);
    assert_eq!(result, false);
    let result = is_palindrome(1221);
    assert_eq!(result, true);
    let result = is_palindrome_1(10);
    assert_eq!(result, false);
    let result = is_palindrome_1(21120);
    assert_eq!(result, true);
}
