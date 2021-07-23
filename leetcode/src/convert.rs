#[test]
pub fn test_conver() {
    let result = convert(String::from("PAYPALISHIRING"), 3);
    println!("result: {}", result);
    assert_eq!(result, String::from("PAHNAPLSIIGYIR"));
}

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
