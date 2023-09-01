use std::sync::Mutex;

lazy_static::lazy_static! { static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(43453)); }

pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().next_value(max)
}

pub struct RandGen {
    current: usize,
    multiplier: usize,
    increment: usize,
    modulus: usize,
}

impl RandGen {
    pub fn new(current: usize) -> Self {
        RandGen { current, multiplier: 43632341, increment: 443533, modulus: 283647 }
    }

    pub fn next_value(&mut self, max: usize) -> usize {
        self.current = (self.current * self.multiplier + self.increment) % self.modulus;
        self.current % max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rands() {
        let mut r = RandGen::new(12);
        for _ in 0..100 {
            println!("{}", r.next_value(100));
        }
    }
}
