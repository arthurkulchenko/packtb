use core::ops::AddAssign;

pub trait Rangable: PartialOrd+std::ops::AddAssign+Copy {}
// WHY: do we need need impl statement here?
impl<T:AddAssign + PartialOrd + Copy> Rangable for T {}

pub struct GenRangeIterator<T> {
    current: T,
    stop: T,
    step: T
}

impl <T> GenRangeIterator<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        GenRangeIterator { current: start, stop, step }
    }
}

impl <T:Rangable> Iterator for GenRangeIterator<T> {
    type Item = T;
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
    fn gen_sum_of_range() {
        let mut m = 0.0;
        let it = GenRangeIterator::new(5.0,12.0,3.0);
        for s in it {
            println!("{}", s);
            m += s;
        }
        assert_eq!(m, 5.0+8.0+11.0, "float Sum of range");
    }
}
