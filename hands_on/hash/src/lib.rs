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

    fn get<KB>(&self, k: &KB) -> Option<&V> where K: Borrow<KB>, KB: Hash + Eq + ?Sized  {
        let hash_index = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (inner_key, inner_value) in &self.buckets[hash_index] {
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

#[derive(Debug)]
pub struct Hmap<K, V> {
    n_moved: usize,
    main: BucketList<K,V>,
    aux: BucketList<K,V> // grow
}

const BSIZE: usize = 8;
const BAUXSIZE: usize = 8;

impl <K: Hash + Eq, V> Hmap<K,V> {
    pub fn new() -> Self {
        Self { n_moved: 0, main: BucketList::new(), aux: BucketList::new() }
    }

    pub fn insert(&mut self, k: K, v: V) {
        if let Some(inner_value) = &mut self.main.get_mut(&k) {
            *inner_value = &v;
            return;
        }
        if let Some(inner_value) = &mut self.aux.get_mut(&k) {
            *inner_value = &v;
            return;
        }
        if self.n_moved > 0 {
            self.aux.push(k, v);
            self.move_buacket();
            return;
        }
        if self.main.push(k, v) > BSIZE / 2 {
            self.move_buacket();
        }
    }

    pub fn get_mut<KR>(&mut self, kr: &mut KR) -> Option<& V> where K: Borrow<KR>, KR: Hash + Eq + ?Sized {
        if let Some(inner_value) = self.main.get_mut(kr) { return Some(inner_value); }
        self.aux.get_mut(kr)
    }

    pub fn get<KR>(&self, kr: &KR) -> Option<&V> where K: Borrow<KR>, KR: Hash + Eq + ?Sized {
        self.main.get(kr).or_else(|| self.aux.get(kr))
    }

    pub fn len(&self) -> usize {
        self.main.buckets.len() + self.aux.buckets.len()
    }

    pub fn move_buacket(&mut self) {
        if self.n_moved == 0 {
            self.aux.set_buckets(self.main.buckets.len() * BAUXSIZE);
        }
        if let Some(inner_value) = self.main.bucket_pop(self.n_moved) {
            for (k, v) in inner_value {
                self.aux.push(k, v);
            }
            self.n_moved += 1;
            return;
        }
        std::mem::swap(&mut self.main, &mut self.aux);
        self.n_moved = 0;
    }
}











