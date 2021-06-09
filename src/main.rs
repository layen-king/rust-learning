#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::{RwLock};

lazy_static! {
    static ref MAP: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

#[derive(Default)]
struct Cache {
    map: RwLock<HashMap<String, String>>,
}

impl Cache {
    fn read(&self, path: &str) -> String {
        let r = self.map.read().unwrap();
        let default = String::from("");
        let f = r.get(path).unwrap_or(&default);
        f.to_string()
    }
    fn write(&mut self, key: &str, value: &str) {
        let mut w = self.map.write().unwrap();
        w.insert(key.to_string(), value.to_string());
    }
}

fn main() {
    let mut h = Cache {
        ..Default::default()
    };
    h.write("k1", "v1");
    {
        let value = h.read("k1");
        println!("{:?}", value);
    }
    {
        println!("{:?}", h.read("k2"));
    }
}
