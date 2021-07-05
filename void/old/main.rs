mod add;
mod function;
mod if_else;
mod index;
mod slic;
mod kind;
mod stu;
mod get_max;

fn main() {
  println!("add is {}", add::cadd(20, 30));
  let location = index::get_index(["a", "b", "c"], "c");
  println!("location is {}", location);
  if_else::if_else();
  function::quote();
  function::var_quote();
  slic::slice_arr();
  stu::rect();
  let rect1 = stu::Rect {
    width: 100,
    height: 200,
  };
  println!("rect1 is {}", rect1.area());
  let rect2 = stu::Rect::create(34, 25);
  println!("rect2 is {:?}", rect2);
  kind::kind();
  get_max::get_max_num()
}
