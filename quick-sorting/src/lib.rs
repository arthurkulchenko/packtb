// FIXME. This is not a proper implementation of quick sort.
mod b_rand;

// TODO: FIX
// TODO: Add comments
pub fn pivot<T: PartialOrd>(v: &mut[T] ) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    // let mut p = 0;
    for i in 1..(v.len() - 1) {
        if v[i] < v[p] {
            v.swap(p+1, i);
            v.swap(p, p+1);
            p += 1
        }
    }
    p
}

// TODO: Add comments
pub fn quick_sort<T: PartialOrd>(v: &mut [T]) {
    if v.len() > 1 {
        let p = pivot(v);
        // quick_sort(&mut v[..p]);
        // quick_sort(&mut v[p+1..]);
        let (a, b) = v.split_at_mut(p);
        quick_sort(a);
        quick_sort(&mut b[1..]);
    }
}

// struct RawSend<T>(*mut [T]);
// unsafe impl<T> Send for RawSend<T> {}

// pub fn threded_quick_sort<T: PartialOrd + Send>(v: & mut [T]) {
// NOTICE: 'static lifetime seems to do the thing without raw pointer
pub fn threded_quick_sort<T: 'static + PartialOrd + Send>(v: &mut [T]) {
    if v.len() > 1 {
        let p = pivot(v);
        // quick_sort(&mut v[..p]);
        // quick_sort(&mut v[p+1..]);
        let (a, b) = v.split_at_mut(p);

        let handle = std::thread::spawn(move || threded_quick_sort(a));
        // let raw_p: *mut [T] = a as *mut [T];
        // let raw_a = RawSend(raw_p);
        // let handle = std::thread::spawn(move || threded_quick_sort(&mut *raw_a.0));

        threded_quick_sort(b);
        // quick_sort(a);
        // quick_sort(&mut b[1..]);
        handle.join().ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_test() {
        let mut v = vec![13, 4, 6, 1, 8, 11, 3];
        // quick_sort(&mut v);
        threded_quick_sort(&mut v);
        assert_eq!(v, [1, 3, 4, 6, 8, 11, 13]);
    }
}
