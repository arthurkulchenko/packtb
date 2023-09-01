pub fn merge_sort<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }
    // Each branch local result
    let mut local_accumulator = Vec::with_capacity(v.len()); // 1
    // NOTICE: split_off mutates 'v' by cutting of second half exclusively [0, at) and rest pub into 'b' [at, len).
    let mut b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    b = merge_sort(b);

    // Inner branch work
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();
    loop {
        // OPTIMIZE: Get rid of pattern spagetti
        match a_peek {
            // checking value presence of a first half
            Some(ref a_val) => {
                match b_peek {
                    // checking value presence of a second half
                    Some(ref b_val) => {
                        if b_val < a_val {
                            // progress B, smaller then A
                            local_accumulator.push(b_peek.take().unwrap());
                            b_peek = b_it.next();
                        } else {
                            // progress A, smaller then B
                            local_accumulator.push(a_peek.take().unwrap());
                            a_peek = a_it.next();
                        }
                    },
                    None => {
                        // progress A, no more B
                        // we need push a_peek because it is no longer part of iterator
                        local_accumulator.push(a_peek.take().unwrap());
                        // Already sorted by the nature of an algorithm
                        local_accumulator.extend(a_it);
                        return local_accumulator;
                    }
                }
            },
            None => {
                // progress B, no more A
                // we need push b_peek because it is no longer part of iterator
                if let Some(b_val) = b_peek { local_accumulator.push(b_val); }
                // Already sorted by the nature of an algorithm
                local_accumulator.extend(b_it);
                return local_accumulator;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![13, 4, 6, 1, 8, 11, 3];
        let sorted = merge_sort(v.clone());
        assert_eq!(sorted, [1, 3, 4, 6, 8, 11, 13]);
    }
}
