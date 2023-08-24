mod gen_range;

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
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.stop { return None; }

        let res = self.current;
        self.current += self.step;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_range() {
        let mut m = 0;
        let it = RangeIterator::new(5,12,3);
        for s in it {
            m += s;
        }
        assert_eq!(m, 4+8+12, "Sum of range");
    }
}
