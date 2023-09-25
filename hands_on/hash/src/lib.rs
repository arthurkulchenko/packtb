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
        let hash_index = (hash(self.seed, &key) as usize) % self.buckets.len();
        self.buckets[hash_index].push((key, value));
        self.length += 1;
        self.buckets[hash_index].len()
    }

    fn get_mut<KB>(&mut self, k: &KB) -> Option<&V> where K: Borrow<KB>, KB: Hash + Eq + ?Sized  {
        let hash_index = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (inner_key, inner_value) in &mut self.buckets[hash_index] {
            if k == (inner_key as &K).borrow() { return Some(inner_value) }
        }
        None
    }

    fn bucket_pop(&mut self, index: usize) -> Option<Vec<(K,V)>> {
        if index >= self.buckets.len() { return None }

        let mut result = Vec::new();
        std::mem::swap(&mut result, &mut self.buckets[index]);
        self.length -= result.len();
        Some(result)
    }

    fn set_buckets(&mut self, index: usize) {
        for _ in self.buckets.len()..index {
            self.buckets.push(Vec::new());
        }
    }
}
