#![forbid(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    table: HashMap<K, V>,
    key_queue: VecDeque<K>,
    cap: usize,
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("cap is zero");
        }
        Self {
            table: HashMap::with_capacity(capacity),
            key_queue: VecDeque::with_capacity(capacity),
            cap: capacity,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.table.get(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // println!("len = {}, cap = {}", self.table.len(), self.cap);
        if !self.table.contains_key(&key) && !self.is_full() {
            // println!("insert {}", self.table.capacity());
            if self.key_queue.len() == 0 {
                self.key_queue.push_back(key.clone());
            }
            self.table.insert(key.clone(), value);
        } else if !self.table.contains_key(&key) && self.is_full() {
            // println!("2 if:");
            let min_key = self.key_queue.pop_back();
            self.table.remove(&min_key.unwrap());
            self.table.insert(key, value);
        } else if self.table.contains_key(&key) {
            return self.table.insert(key.clone(), value);
        }
        // println!("-----");
        None
    }

    fn is_full(&self) -> bool {
        self.table.len() == self.cap
    }

}
