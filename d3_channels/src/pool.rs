use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time};

use time::Duration;
use thread::sleep;

pub struct ThreadPool {
    sink: Option<mpsc::Sender::<Box<dyn Fn() + Send>>>,
    amount: u32,
    done: mpsc::Receiver<()>,
}

impl ThreadPool {
    pub fn new(amount: u32) -> Self {
        let (sink, stream) = mpsc::channel();
        let ts_stream = Arc::new(Mutex::new(stream));

        let (done_tx, done_rx) = mpsc::channel();
        for _ in 0..amount {
            let stream = ts_stream.clone();
            let done_sink = done_tx.clone();

            thread::spawn(move || loop {
                let m = stream.lock().unwrap();
                let f: Box<dyn Fn() + Send> = match m.recv() {
                    Ok(f) => f,
                    Err(_) => {
                      done_sink.send(()).ok();
                      return;
                    },
                };
                // Ensure that the mutex is dropped before the function is called
                // so function won't hold us back from parallelizing
                drop(m);
                f();
            });
        }
        // Self { sink: sink, amount }
        Self { sink: Some(sink), done: done_rx, amount: amount }
    }

    pub fn run<F: Fn() + Send + 'static>(&self, f: F) {
        // if let Some(ref sink) = self.sink {};
        match &self.sink {
            Some(sink) => sink.send(Box::new(f)).unwrap(),
            None => panic!("No thread pool"),
        }
        
    }

    pub fn wait(mut self) {
        // sleep(Duration::from_millis(3000));
        self.sink.take();
        for _ in 0..self.amount {
            self.done.recv().unwrap();
        }
    }
}

fn main() {
    let thread_pool = ThreadPool::new(10);
    let tasks = 0..100;
    for task in tasks {
        thread_pool.run(move || { compute_in_threads(task); })
    }
    thread_pool.wait();
}

pub fn compute_in_threads(task: u32) {
    sleep(Duration::from_millis(100));
    println!("run = {}", task);
}
