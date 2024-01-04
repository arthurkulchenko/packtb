use std::sync::Condvar;
use std::collections::VecDeque;
use std::sync::Mutex;

fn main() {
    println!("Hello, world!");
}

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    is_ready: Condvar,
}

impl <T> Channel<T> {
    fn new() -> Self {
        Self { queue: Mutex::new(VecDeque::new()), is_ready: Condvar::new() }
    }

    fn send(&self, message: T) {
        // NOTICE: Because lock not assigned to a variable lock is droped in the nex line.
        self.queue.lock().unwrap().push_back(message);
        self.is_ready.notify_one();
    }

    fn receive(&self) -> T {
        let mut guard = self.queue.lock().unwrap();
        loop {
            if let Some(message) = guard.pop_front() {
                return message;
            }
            // NOTICE: #wait method releases mutex and blocks thread until notified.
            guard = self.is_ready.wait(guard).unwrap();
        }
    }
}
