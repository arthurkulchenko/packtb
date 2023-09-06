use std::future::Future;
use std::task::{Context, Poll};

use std::pin::Pin;

pub struct SimpleFuture {
    count: usize,
}

impl Future for SimpleFuture {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        if self.count > 0 {
            self.count -= 1;
            println!("count: {}", self.count);
            Poll::Pending
        } else {
            Poll::Ready(42)
        }
    }
}


// #[cfg(test)]
// mod specs {
//     use super::*;
// }
