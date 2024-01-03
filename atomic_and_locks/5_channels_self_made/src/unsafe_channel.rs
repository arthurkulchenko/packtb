use std::mem::MaybeUninit;
use std::cell::UnsafeCell;

use std::sync::atomic::AtomicBool;
use std::sync::Ordering::{Release, Acquire};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    /// Safty: Only call this once!
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Acquire)
    }

    /// Sefety: Only call this once and only after is_ready() returns true!
    pub unsafe fn receive(&self) -> T {
        // NOTICE: Assumes it has already been initialuzed and that it isn't being used to produce multiple copies
        // of non-Copy objects.
        (*self.message.get()).assume_init()
    }
}
