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
        // if self.buckets[hash_index].len() == 1 {
        //     self.buckets[0].push((key, value));
        // } else {
            self.buckets[hash_index].push((key, value));
        // }
        self.length += 1;
        self.buckets[hash_index].len()
    }

    // NOTICE: Rework ====================
    fn get_mut<KB>(&mut self, k: &KB) -> Option<&V> where K: Borrow<KB>, KB: Hash + Eq + ?Sized  {
        let hash_index = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (inner_key, inner_value) in &mut self.buckets[hash_index] {
            if k == (inner_key as &K).borrow() { return Some(inner_value) }
        }
        None
    }

    // NOTICE: Rework ====================
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

impl <K: Hash + Eq, V: std::fmt::Debug> Hmap<K,V> {
    pub fn new() -> Self {
        let instance = Self { n_moved: 0, main: BucketList::new(), aux: BucketList::new() };
        println!("~~~~~~~~~ New Hmap instance len() is {:?}", instance.len());
        instance
    }

    pub fn insert(&mut self, k: K, v: V) {
        if let Some(inner_value) = &mut self.main.get_mut(&k) {
            println!("~~~~~~~~~ Updating value - {:?}, to value - {:?}", inner_value, v);
            *inner_value = &v;
            // DEBUG: Sets value right, but specs don't pass
            println!("~~~~~~~~~ New value is {:?}", inner_value);
            return;
        }
        if let Some(inner_value) = &mut self.aux.get_mut(&k) {
            *inner_value = &v;
            return;
        }
        if self.n_moved > 0 {
            self.aux.push(k, v);
            self.move_bucket();
            return;
        }
        if self.main.push(k, v) > BSIZE / 2 {
            self.move_bucket();
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

    pub fn move_bucket(&mut self) {
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

#[cfg(test)]
mod test {
    use crate::Hmap;

    #[test]
    fn get_right_values() {
        let mut hm = Hmap::new();
        assert_eq!(hm.len(), 2);

        hm.insert("string1".to_string(), 4);
        hm.insert("string2".to_string(), 3);
        hm.insert("string3".to_string(), 5);
        hm.insert("string4".to_string(), 22);
        hm.insert("string5".to_string(), 1);
        hm.insert("string6".to_string(), 0);

        assert_eq!(hm.get("string1"), Some(&4));
        assert_eq!(hm.get("string2"), Some(&3));
        assert_eq!(hm.get("string3"), Some(&5));
        assert_eq!(hm.get("string4"), Some(&22));
        assert_eq!(hm.get("string5"), Some(&1));
        assert_eq!(hm.get("string6"), Some(&0));

        hm.insert("string7".to_string(), 9);
        assert_eq!(hm.get("string7"), Some(&9));

        hm.insert("string7".to_string(), 3);
        assert_eq!(hm.get("string7"), Some(&3));

        assert_eq!(hm.len(), 7 + 2);
    }
}
