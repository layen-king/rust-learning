pub struct EventEmiter {}
impl EventEmiter {
    #[allow(unused)]
    pub fn register(event: String, callback: Box<fn()>) {}
    #[allow(unused)]
    pub fn emit(event: String) {}
    /// ## 移除事件注册
    /// ### [event:String]  事件名称
    /// ### [callback] 注册的函数
    /// ```
    /// unregister(string::from("test"), fn)
    /// ```
    #[allow(unused)]
    pub fn unregister(event: String, callback: Box<fn()>) {}
    pub fn clear_registered() {}
}

#[test]
fn test() {
    let a = String::from("abc");
    println!("a ptr: {:?}, a :{:?}", a.as_ptr(), a)
}
