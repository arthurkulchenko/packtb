// use std::future::Future;
use std::task::{Context, Poll};
use futures::future::Future;



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
            Poll::Ready(41)
        }
    }
}

async fn plus_one(n: i32) -> i32 {
    n + 1
}


#[cfg(test)]
mod specs {
    // NOTICE: allow use of map on a future
    use futures::FutureExt;
    use futures::task::noop_waker;
    use super::*;
    use futures::executor::block_on;
    use futures::channel::oneshot;

    #[test]
    fn test_future() {
        // let mut future = SimpleFuture { count: 10 };
        // let mut context = Context::from_waker(futures::task::noop_waker_ref());
        // assert_eq!(Pin::new(&mut future).poll(&mut context), Poll::Pending);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = SimpleFuture { count: 1 };
        match Pin::new(&mut future).poll(&mut context) {
            Poll::Pending => assert!(true), // this is what we expect
            Poll::Ready(_) => assert!(false),
        }
    }

    #[test]
    fn future_returns_values() {
        // let f = SimpleFuture { count: 0 };
        let f = plus_one(42);
        let (sink, stream) = oneshot::channel();
        let _ = block_on(f.map(move |n| sink.send(n - 1)));
        let result = block_on(stream);
        // Pin::new(&mut f).poll(&mut std::task::Context<_>);
        assert_eq!(result, Ok(42));
    }
}
