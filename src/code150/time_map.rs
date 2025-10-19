use std::collections::*;

type Target = (i32, String);
type UseValue = i32;
fn lower_bound(arr: &[Target], x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        match arr[mid].0.cmp(x) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}

pub struct TimeMap {
    dict: HashMap<String, Vec<(i32, String)>>,
}
impl TimeMap {
    pub fn new() -> Self {
        TimeMap {
            dict: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.dict
            .entry(key)
            .or_default()
            .push((timestamp, value));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(arr) = self.dict.get(&key) {
            let ti = lower_bound(arr, &timestamp);
            let len = arr.len();

            if ti == arr.len() {
                arr[len - 1].1.clone()
            } else if arr[ti].0 == timestamp {
                arr[ti].1.clone()
            } else if ti == 0 {
                String::new()
            } else {
                arr[ti - 1].1.clone()
            }
        } else {
            String::new()
        }
    }
}
