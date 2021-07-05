use std::fs::File;
use std::io;
use std::io::Read;

fn read_text(path: &str) -> Result<String, io::Error> {
  let mut f = File::open(path)?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  return Ok(s);
}

pub fn kind() {
  let str_file = read_text("hello.txt");
  match str_file {
    Ok(s) => println!("s is {}", s),
    Err(e) => match e.kind() {
      io::ErrorKind::NotFound => println!("No such file"),
      _ => println!("cannot read the file"),
    },
  }
}
