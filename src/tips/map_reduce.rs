use std::sync::mpsc::channel;
use std::thread;
pub fn run_map_rudece() {
    let data = "1 21 dfsa1 3424 21 32 789789 45456 48456";
    let chunk_data = data.split_whitespace();
    let (tx, rx) = channel();
    let mut index = 0;
    // 使用多线程计算
    for (i, el) in chunk_data.enumerate() {
        index += 1;
        println!("{} is {}", i, el);
        let tx = tx.clone();
        thread::spawn(move || {
            let result = el
                .chars()
                .filter_map(|c| String::from(c).parse().ok())
                .fold(0, |acc: i32, x: i32| acc + x);
            tx.send(Box::new(result))
        });
    }
    let mut arr = vec![];
    for _ in 0..index {
        let n = rx.recv().expect("is Number");
        arr.push(*n)
    }
    arr.sort();
    println!("sort is:{:?}, count is {}", arr, arr.iter().sum::<i32>());

    println!("data: {:?}", data);
    // 遍历字符相加
    let mut count: i128 = 0;
    for (_i, el) in data.chars().enumerate() {
        let res: Result<i128, _> = String::from(el).parse();
        match res {
            Ok(n) => count = count + n,
            Err(_) => {}
        }
    }
    println!("count is max:{}", count);

    string_spili();
}

/// 使用多线程分割固定次数,并且存储子线程
fn string_spili() {
    let data = "1 21 dfsa1 3424 21 32 789789 45456 48456";
    let data: Vec<i32> = data
        .chars()
        .filter_map(|f| String::from(f).parse().ok())
        .collect();
    println!("data is {:?}", data);
    let mut children = vec![];
    let mut index = 0;
    let mut count = 0;
    let (tx, rx) = channel();
    loop {
        if index >= data.len() + 5 {
            break;
        }
        println!("index is {},len is {}", index, data.len());
        // 取出字符
        let mut char_vec = vec![];
        for i in index..index + 5 {
            let n = data.get(i);
            if let Some(i) = n {
                char_vec.push(*i)
            }
        }
        // 如果char_vec为正常字符
        println!("char vec is {:?},", char_vec);
        let tx = tx.clone();
        children.push(thread::spawn(move || {
            let result = char_vec.iter().fold(0, |acc, &x| acc + x);
            tx.send(result)
        }));
        let n = rx.recv().expect("not a number");
        count += n;
        index += 5;
    }
    println!("count is : {}", count);
}
