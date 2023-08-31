pub fn pivot<T: PartialOrd>(v: &mut[T] ) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p+1, i);
            v.swap(p, p+1);
            p += 1
        }
    }
    p
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_test() {
        let mut v = vec![13, 4, 6, 1, 8, 11, 3];
        quick_sort(&mut v);
        assert_eq!(v, [1, 3, 4, 6, 8, 11, 13]);
    }
}
