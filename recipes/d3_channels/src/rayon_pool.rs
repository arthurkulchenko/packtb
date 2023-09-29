use rayon::prelude::*;
use rayon::join;
use std::marker::{Send, Sync};

fn main() {
    let mut v = Vec::with_capacity(100);
    for i in 0..100 {
        v.push(i);
    }

    on_each(
        &mut v,
        |n| {
            println!("doing: {}", n);
            std::thread::sleep(std::time::Duration::from_millis(100));
            *n += 5;
        }
    );
    println!("result = {:?}", v);
    v.par_iter_mut().for_each(|n| {
        println!("doing: {}", n);
        *n -= 5;
    });
    println!("final result = {:?}", v);
}

pub fn on_each<T, F>(v: &mut [T], f: F) where F: Fn(&mut T) + Send + Sync + Copy, T: Send + Sync {
    match v.len() {
        0 => return,
        n if (n < 4) => {
            for i in v {
                f(i)
            }
        },
        n => {
            let (v1, v2) = v.split_at_mut(n/2);
            join(|| on_each(v1, f), || on_each(v2, f));
        }
    }
}
