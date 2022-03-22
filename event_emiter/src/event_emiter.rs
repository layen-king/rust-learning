use bincode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use uuid::Uuid;
#[allow(dead_code)]
pub struct Listener {
    callback: Arc<dyn Fn(Vec<u8>) + Sync + Send + 'static>,
    limit: Option<u64>,
    id: String,
}

/// ## 事件发布订阅
#[derive(Default)]
pub struct EventEmitter {
    listeners: HashMap<String, Vec<Listener>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
    /// ## 处理事件次数
    fn on_limited<F, T>(&mut self, event: &str, limit: Option<u64>, callback: F) -> String
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T) + 'static + Sync + Send,
    {
        let id = Uuid::new_v4().to_string();
        let parse_callback = move |bytes: Vec<u8>| {
            let value: T = bincode::deserialize(&bytes).unwrap();
            callback(value);
        };
        let listener = Listener {
            id: id.clone(),
            limit,
            callback: Arc::new(parse_callback),
        };
        match self.listeners.get_mut(event) {
            Some(callbacks) => {
                callbacks.push(listener);
            }
            None => {
                self.listeners.insert(event.to_string(), vec![listener]);
            }
        }
        id
    }
    /// ## 注册事件
    /// ### [event] 事件名称
    /// ### [callback] 事件回调
    /// ### [return] 返回事件id
    pub fn register<F, T>(&mut self, event: &str, callback: F) -> String
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T) + 'static + Send + Sync,
    {
        let id = self.on_limited(event, None, callback);
        id
    }
    /// ## 触发一次
    pub fn once<F, T>(&mut self, event: &str, callback: F) -> String
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T) + 'static + Send + Sync,
    {
        let id = self.on_limited(event, Some(1), callback);
        id
    }
    /// ## 触发事件
    /// ### [event] 事件名称
    /// ### [value] 触发参数
    pub fn emit<T>(&mut self, event: &str, value: T) -> Vec<thread::JoinHandle<()>>
    where
        T: Serialize,
    {
        let mut callback_handlers = vec![];
        if let Some(listeners) = self.listeners.get_mut(event) {
            let bytes = bincode::serialize(&value).unwrap();

            let mut listeners_to_remove = vec![];
            for (index, listener) in listeners.iter_mut().enumerate() {
                let clone_bytes = bytes.clone();
                let callback = Arc::clone(&listener.callback);
                match listener.limit {
                    None => callback_handlers.push(thread::spawn(move || callback(clone_bytes))),
                    Some(limit) => {
                        if limit != 0 {
                            callback_handlers.push(thread::spawn(move || callback(clone_bytes)));
                            listener.limit = Some(limit - 1);
                        } else {
                            listeners_to_remove.push(index);
                        }
                    }
                }
            }

            // 多线程移除执行完毕的
            for index in listeners_to_remove.into_iter().rev() {
                listeners.remove(index);
            }
        } else {
            println!("event :{} not found", event);
        }
        callback_handlers
    }
    /// ## 移除事件注册
    pub fn remove_registered(&mut self, id: &str) -> bool {
        for (_, listeners) in self.listeners.iter_mut() {
            if let Some(index) = listeners.iter().position(|listener| listener.id == id) {
                listeners.remove(index);
                return true;
            }
        }
        false
    }
    pub fn get_count(&self) -> usize {
        self.listeners.len()
    }
}

#[test]
fn test() {
    let mut event_emiter = EventEmitter::new();
    fn t(a: String) {
        println!("a :{}", a);
    }
    event_emiter.register("hello", t);
    event_emiter.emit("hello", "test");
    event_emiter.once("hello1", t);
    event_emiter.emit("hello1", "abcd");
    event_emiter.emit("hello1", "abcde");
    println!("{}", event_emiter.get_count());
    event_emiter.emit("hello1", "abcde");
}
