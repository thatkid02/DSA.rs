use std::collections::HashMap;

#[derive(Clone)]
struct Node {
    key: i32,
    value: i32,
    new: usize,
    old: usize,
}

struct LeastRecentlyUsedCache {
    nodes: Box<[Node]>,
    node_by_key: HashMap<i32, usize>,
    head: usize,
}

impl LeastRecentlyUsedCache {
    fn new(capacity: usize) -> Self {
        let mut cache = Self {
            node_by_key: HashMap::with_capacity(capacity),
            nodes: vec![
                Node {
                    key: 0,
                    value: 0,
                    new: 0,
                    old: 0,
                };
                capacity
            ]
            .into_boxed_slice(),
            head: capacity - 1,
        };

        for i in 1..capacity {
            cache.nodes[i].new = i + 1;
            cache.nodes[i].old = i - 1;
        }

        cache.nodes[0].new = 1;
        cache.nodes[0].old = capacity - 1;
        cache.nodes[capacity - 1].new = 0;
        cache
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        match self.node_by_key.get(&key) {
            Some(addr) => {
                let val = self.nodes[*addr].value;
                if *addr == self.head {
                    Some(val)
                } else if *addr == self.nodes[self.head].new {
                    self.add_new(key, None);
                    Some(val)
                } else {
                    self.update_non_head_or_tail(*addr, None);
                    Some(val)
                }
            }
            None => None,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.node_by_key.get(&key) {
            Some(addr) => {
                if *addr == self.head {
                    self.nodes[self.head].value = value;
                } else if *addr == self.nodes[self.head].new {
                    self.add_new(key, Some(value));
                } else {
                    self.update_non_head_or_tail(*addr, Some(value));
                }
            }
            None => self.add_new(key, Some(value)),
        }
    }

    fn update_non_head_or_tail(&mut self, addr: usize, maybe_new_value: Option<i32>) {
        match maybe_new_value {
            Some(value) => self.nodes[addr].value = value,
            None => (),
        }
        let tail_addr = self.nodes[self.head].new;

        let updated = &mut self.nodes[addr];
        let updated_old = updated.old;
        let updated_new = updated.new;
        self.nodes[updated_old].new = updated_new;
        self.nodes[updated_new].old = updated_old;

        let updated = &mut self.nodes[addr];
        updated.old = self.head;
        updated.new = tail_addr;
        self.nodes[self.head].new = addr;
        self.nodes[tail_addr].old = addr;

        self.head = addr
    }

    fn add_new(&mut self, key: i32, value: Option<i32>) {
        self.head = self.nodes[self.head].new;

        let node_to_replace = &mut self.nodes[self.head];

        self.node_by_key.remove(&node_to_replace.key);
        self.node_by_key.insert(key, self.head);

        node_to_replace.key = key;
        match value {
            Some(value) => node_to_replace.value = value,
            None => (),
        }
    }
}

pub struct LRUCache {
    cache: LeastRecentlyUsedCache,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            cache: LeastRecentlyUsedCache::new(capacity as usize),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.cache.get(key).unwrap_or(-1)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.cache.put(key, value);
    }
}
