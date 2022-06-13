#[warn(unused_must_use)]
pub async fn do_something() {
    println!("go go go!");
}

#[test]
fn test_do_something() {
  do_something();
}

