mod code150;

use crate::code150::time_map::TimeMap;

fn main() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        let value = time_map.get("foo".to_string(), 1);
        println!("Value at timestamp 1: {}", value);
        let value2 = time_map.get("foo".to_string(), 3);
        println!("Value at timestamp 3: {}", value2);
}
