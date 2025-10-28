mod code150;

fn main() {
    use code150::lru_cache::LRUCache;

    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("Get 1: {}", cache.get(1)); 
    cache.put(3, 3);
    println!("Get 2: {}", cache.get(2));
    cache.put(4, 4);
    println!("Get 1: {}", cache.get(1)); 
    println!("Get 3: {}", cache.get(3)); 
    println!("Get 4: {}", cache.get(4));
}
