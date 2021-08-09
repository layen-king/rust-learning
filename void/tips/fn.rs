pub fn call_with_one<F>(func: F) -> isize
where
    F: Fn(isize) -> isize,
{
    func(100)
}

#[test]
fn test_call_with_one() {
    let double = |x: isize| x * 2;
    let result = call_with_one(double);
    println!("{:?}", result);
}
