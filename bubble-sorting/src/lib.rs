// pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
//     let array_size = v.len() - 1;
//     for _ in 0..array_size {
//         for el in 0..array_size {
//             if v[el] > v[el + 1] {
//                 v.swap(el, el + 1)
//             }
//         }
//     }
// }

// NOTICE: adding 'sorted' variable we avoid unnecessary iterations of outter loop. but still worst scenario O(n^2).
// pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
//     let array_size = v.len() - 1;
//     for _ in 0..array_size {
//         let mut sorted = true;
//         for el in 0..array_size {
//             if v[el] > v[el + 1] {
//                 v.swap(el, el + 1);
//                 sorted = false;
//             }
//         }
//         if sorted { return; }
//     }
// }

// Use of static dispatch
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    let array_size = v.len() - 1;
    for t in 0..array_size {
        // NOTICE: adding 'sorted' variable we avoid unnecessary iterations of outter loop. but still worst scenario O(n^2).
        let mut sorted = true;
        // NOTICE: reducing the number of iterations by 't' of the inner loop, we can improve the performance of the algorithm,
        //   since after first run of the inner loop, the last element of the array is already sorted due to nature of algorithm.
        let reduced_size = array_size - t;
        for el in 0..reduced_size {
            if v[el] > v[el + 1] {
                v.swap(el, el + 1);
                sorted = false;
            }
        }
        if sorted { return; }
    }
}

// TODO: implement bubble sort with iterators
// TODO: improve algorithm by changing iteration direction of inner loop to get smallest in place in one iteration and again
//   reduce inner loop iterations by 't + 0,5' of the outter loop, since we can reduce iteration by 2 only after when we went back.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![13, 4, 6, 1, 8, 11, 3];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 3, 4, 6, 8, 11, 13]);
    }
}
