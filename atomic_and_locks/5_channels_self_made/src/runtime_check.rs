use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::thread;

use std::sync::atomic::AtomicBool;
use std::sync::Ordering::{Release, Acquire, Relaxed};

fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            t.unpark();
        })
        while !channel.is_ready() {
            thread.park();
        }
        assert_eq!(channel.receive(), "hello world!");
    });
}

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    in_use: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { (*self.message.get_mut()).assume_init_drop() };
        }
    }
}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            in_use: AtomicBool::new(false),
        }
    }

    /// Safty: Only call this once!
    pub unsafe fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) { panic!("can't send more then once message at a time!") }

        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    /// Sefety: Only call this once and only after is_ready() returns true!
    pub fn receive(&self) -> T {
        // NOTICE: Swap returns the previous value of the flag. If it was false, then we panic.
        if !self.ready.swap(false, Acquire) { panic!("no message available!"); }

        // Safety: We have just checked (adn reset) the ready flag.
        unsafe { (*self.message.get()).assume_init_read() }
    }
}
