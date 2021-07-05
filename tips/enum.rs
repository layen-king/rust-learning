#[allow(dead_code)]
/// 无参数枚举
enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}
#[allow(dead_code)]#[derive(Debug)]
/// 类c枚举
enum Color {
    Red = 0xff000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
#[derive(Debug)]#[allow(dead_code)]
/// 带参枚举
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}



/// 执行枚举函数
pub fn run_emum() {
    let a = Number::Zero;
    match a {
        Number::One => println!("a is 1"),
        _ => println!("number is other"),
    }

    println!("color is {:?}", Color::Blue);

    // let x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
    // let y: fn(String) -> IpAddr = IpAddr::V6;
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("ip addr is {:?}", home)

    
}
