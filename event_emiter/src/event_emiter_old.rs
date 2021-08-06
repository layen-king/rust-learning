use std::collections::HashMap;
struct Event {
    method: Box<fn()>,
    once: bool,
}

pub struct EventEmiter {
    count: usize,
    events: HashMap<String, Vec<Event>>,
}
impl EventEmiter {
    #[allow(unused_mut)]
    pub fn register(&mut self, event_type: String, callback: Box<fn()>, once: bool) {
        if self.events.contains_key(&event_type) {
            let mut vec = self.events.get(&event_type).unwrap();
            vec.push(Event {
                method: callback,
                once,
            })
        } else {
            let mut event = Event {
                method: callback,
                once,
            };
            let mut vec = vec![event];
            self.events.insert(event_type, vec);
        }
    }
    #[allow(unused)]
    pub fn emit(event: String) {}
    /// ## 移除事件注册
    /// ### [event:String]  事件名称
    /// ### [callback] 注册的函数
    /// ```
    /// unregister(string::from("test"), fn)
    /// ```
    #[allow(unused)]
    pub fn unregister(self: &mut Self, event_type: String, callback: Box<fn()>, once: bool) {}
    pub fn clear_registered() {}
}

#[test]
fn test() {
    fn test() {
        let a = 1;
        println("{}", a);
    }
    EventEmiter::register(String::from('a'), Box::new(test), true);
    println!("{:?}", EventEmiter)
}
