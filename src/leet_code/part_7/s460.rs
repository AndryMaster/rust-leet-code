use std::collections::{HashMap, VecDeque};


#[derive(Debug, Clone)]
struct Node {
    key: i32,
    value: i32,
    freq: usize,
}

#[derive(Debug, Clone)]
pub struct LFUCache {
    capacity: usize,
    min_freq: usize,
    cache: HashMap<i32, Node>,
    freq_map: HashMap<usize, VecDeque<Node>>,
}



impl LFUCache {
    const KEY_NOT_FOUND: i32 = -1;

    pub fn new(capacity: i32) -> Self {
        if !capacity.is_positive() {
            panic!("capacity cannot be negative or zero");
        }

        LFUCache {
            capacity: capacity as usize,
            min_freq: 0,
            cache: HashMap::with_capacity(capacity as usize),
            freq_map: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if !self.cache.contains_key(&key) {
            return Self::KEY_NOT_FOUND;
        }

        let node = &self.cache.get(&key).unwrap().clone();
        self.increment_freq(&node);
        return node.value;
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // If key exists, update its value and increment frequency
        if self.cache.contains_key(&key) {
            let mut entry = self.cache.get(&key).unwrap().clone();
            entry.value = value;

            self.increment_freq(&entry);
            self.cache.insert(key, entry);
            return;
        }

        // If cache is full, evict the least frequently used item
        if self.cache.len() == self.capacity {
            self.compress_cache();
        }

        // Create new node and add it to the cache
        self.min_freq = 1;
        let new_node = Node { key, value, freq: self.min_freq };
        self.add_node_to_freq_list(&new_node);
        self.cache.insert(key, new_node);
    }


    fn increment_freq(&mut self, node: &Node) {
        let current_freq = node.freq;

        // Remove node from current frequency list
        let current_list = self.freq_map.entry(current_freq).or_insert(VecDeque::new());
        current_list.retain(|n| n.key != node.key);

        // If current frequency list becomes empty, clean up
        if current_list.is_empty() {
            self.freq_map.remove(&current_freq);
            if self.min_freq == current_freq {
                self.min_freq += 1;
            }
        }

        // Increment frequency and add to new frequency list
        let node_increment = Node { key: node.key, value: node.value, freq: node.freq + 1 };
        self.add_node_to_freq_list(&node_increment);
        self.cache.entry(node.key).and_modify(|node| {node.freq += 1}); // For test 3
        // Похоже где-то рассинхрон у нодов кеша и списков частот
    }

    fn add_node_to_freq_list(&mut self, node: &Node) {
        self.freq_map.entry(node.freq).or_insert(VecDeque::new()).push_front(node.clone());
    }

    fn compress_cache(&mut self) {
        let list = self.freq_map.entry(self.min_freq).or_insert(VecDeque::new());
        let node = list.pop_back().expect("list of self.min_freq is empty");
        self.cache.remove(&node.key);
    }
}
