#[derive(Debug)]
/// 普通结构体
struct People {
    name: &'static str,
    gender: u32,
}

impl People {
    fn new(name: &'static str, gender: u32) -> People {
        People { name, gender }
    }
    fn name(&self) {
        println!("name is {}", self.name)
    }
    fn gender(&self) {
        let gender = if self.gender == 0 { "boy" } else { "girl" };
        println!("{} sex is {}", self.name, gender)
    }
    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
}

#[derive(Debug)]
/// 元组结构体
struct Color(i32, i32, i32);

/// 执行结构体
pub fn run_struct() {
    let alex = People::new("alex", 0);
    alex.name();
    alex.gender();
    let mut layen = People::new("layen", 0);
    layen.set_name("layen-king");
    layen.name();
    layen.gender();

    let color = Color(255, 255, 255);
    println!("{:?}", color)
}
