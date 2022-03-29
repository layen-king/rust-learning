#[allow(dead_code)]
fn iter_example() {
    let mut vec = vec![1, 2, 3, 4, 5];
    // 不可变借用,只读模式
    for i in vec.iter() {
        print!("i : {}", i);
    }
    // 可变借用,必须vec可变,不然报错, 操作取原值
    for i in vec.iter_mut() {
        *i = *i + 1;
        print!("i : {}", i);
    }
    // 直接获取本身,要变化,必须显示声明mut
    for mut i in vec.into_iter() {
        i = i + 1;
        print!("i :{}", i);
    }
}

#[test]
fn test_iter() {}
