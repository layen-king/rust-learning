/// ## 替换所有的问号

/// ### 简单

/// ### 给你一个仅包含小写英文字母和 '?' 字符的字符串 s，请你将所有的 '?' 转换为若干小写字母，使最终的字符串不包含任何 连续重复 的字符。

/// ### 注意：你 不能 修改非 '?' 字符。

/// ### 题目测试用例保证 除 '?' 字符 之外，不存在连续重复的字符。

/// ### 在完成所有转换（可能无需转换）后返回最终的字符串。如果有多个解决方案，请返回其中任何一个。可以证明，在给定的约束条件下，答案总是存在的。

/// ###  

/// ### 示例 1：

/// ### 输入：s = "?zs"
/// ### 输出："azs"
/// ### 解释：该示例共有 25 种解决方案，从 "azs" 到 "yzs" 都是符合题目要求的。只有 "z" 是无效的修改，因为字符串 "zzs" 中有连续重复的两个 'z' 。


/// ### 示例 2：

/// ### 输入：s = "ubv?w"
/// ### 输出："ubvaw"
/// ### 解释：该示例共有 24 种解决方案，只有替换成 "v" 和 "w" 不符合题目要求。因为 "ubvvw" 和 "ubvww" 都包含连续重复的字符。


/// ### 示例 3：

/// ### 输入：s = "j?qg??b"
/// ### 输出："jaqgacb"


/// ### 示例 4：

/// ### 输入：s = "??yw?ipkj?"
/// ### 输出："acywaipkja"
/// ###  

/// ### 提示：

/// ### 1 <= s.length <= 100

/// ### s 仅包含小写英文字母和 '?' 字符

/// ### 通过次数34,265提交次数66,965

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://eetcode-cn.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn modify_string(s: String) -> String {
  // todo 尚未调试
  let mut v = s.chars().collect::<_>();
  for (i,c) in v.enumerate() {
    if v[i] == '?'{
      if v.get(i-1) !=  Some(&'a') && v.get(i+1) != Some(&'a'){
        v[i] = 'a';
        continue;
      }
      if v.get(i-1) != Some(&'b') && v.get(i+1) != Some(&'b'){
        v[i] = 'b';
        continue;
      }
      if v.get(i-1)!= Some(&'c') &&  v.get(i+1) != Some(&'c'){
        v[i] = 'c';
        continue;
      }
    }
  }
  v.iter().collect::<String>()
}

#[test]
fn test_modify_string(){
  let result = modify_string(String::from("??yw?ipkj?"));
  println!("result:{}",result);
}