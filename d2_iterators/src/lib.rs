mod gen_range;
mod combi;

pub struct RangeIterator {
    current: i32,
    stop: i32,
    step: i32
}

impl RangeIterator {
    pub fn new(start: i32, stop: i32, step: i32) -> Self {
        RangeIterator { current: start, stop, step }
    }
}

impl Iterator for RangeIterator {
    // NOTICE: Item is an associated type with Iterator trait
    //      the Iterator trait in Rust requires you to specify an associated type called Item,
    //      which represents the type that will be yielded during the iteration
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.stop { return None; }

        let result = self.current;
        self.current += self.step;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_range() {
        let mut accum = 0;
        let iterator = RangeIterator::new(5,12,3);
        for el in iterator {
            accum += el;
        }
        assert_eq!(accum, 4+8+12, "Sum of range");
    }
}
