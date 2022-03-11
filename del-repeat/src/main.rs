use std::env;
mod repeat;

fn main(){
  let args:Vec<String> = env::args().collect();
  let mut path = String::from("./");
  if !args.get(1).is_none() {
    path = args.get(1).unwrap().to_string();
  }
  let result = repeat::find_repeat_file(path);
  println!("{:?}",result);
}