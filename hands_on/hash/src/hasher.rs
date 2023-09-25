use std::hash::{Hash, Hasher};

pub struct Mhash {
    prev: u8,
    num: u128,
}

impl Hasher for Mhash {
    fn write(&mut self, data: &[u8]) {
        for datum in data {
            self.num = (((self.num + 11) * (*datum as u128 + 13) + (datum ^ self.prev) as u128)) % (std::u64::MAX as u128);
            self.prev = *datum;
        }
    }

    fn finish(&self) -> u64 {
        self.num as u64
    }
}

pub fn hash<T: Hash>(seed: u64, t: T) -> u64 {
    let mut h = Mhash { num: 0, prev: 0 };
    h.write_u64(seed);
    t.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod test {
    // use::super::*;
    use crate::hasher::hash;

    #[test]
    pub fn test_hasher() {
        let num = hash(55, "cat");
        assert_eq!(num, hash(55, "cat"));
    }
}
