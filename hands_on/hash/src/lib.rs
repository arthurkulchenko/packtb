mod hasher;

use rand;
use std::hash::Hash;
use hasher::hash;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct BucketList<K,V> {
    length: usize,
    buckets: Vec<Vec<(K,V)>>,
    seed: u64
}

impl <K: Hash + Eq, V> BucketList<K,V> {
    fn new() -> Self {
        Self { length: 0, buckets: vec![Vec::new()], seed: rand::random() }
    }

    fn push(&mut self, key: K, value: V) -> usize {
        let h = (hash(self.seed, &key) as usize) % self.buckets.len();
        self.buckets[h].push((key, value));
        self.length += 1;
        self.buckets[h].len()
    }

    fn get<KB>(&self, k: &KB) -> Option<&V> where K: Borrow<KB>, KB: Hash + Eq + ?Sized  {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (inner_key, inner_value) in &self.buckets[h] {
            if k == inner_key.borrow() { return Some(inner_value) }
        }
        None
    }
}
