mod cache;

fn main() {
    let k = "key";
    let v = "value";
    let s = cache::write(k, v);
    println!("writ successfully {}", s);
    let content = cache::read(k);
    println!("content is {}", content)
}
