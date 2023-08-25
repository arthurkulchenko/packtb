pub struct SkipIterator<I:Iterator> {
    inner: I,
}

impl <I,T> Iterator for SkipIterator<I> where I:Iterator<Item = T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()?;
        self.inner.next()
    }
}
