use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::thread;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl <T> Sync for SpinLock<T> where T: Send {}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl <T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

impl <T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self { locked: AtomicBool::new(false), value: UnsafeCell::new(value) }
    }

    pub fn lock(&self) -> Guard<T> {
        // Errorness
        // while self.locked.compare_exchange_weak(false, true, Acquire, Release).is_ok() {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }

}

fn main() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s|{
        // NOTICE: When we call push on the Guard instance, the push method require to obtain ref or mut ref and conmiler
        // will check implementation of its deref and derefmut methods if there are some it will use it to obtain ref or mut ref
        // and there it will find also method chain that leads to actual value. It is kind of hided implementation
        s.spawn(||{ x.lock().push(1); });
        s.spawn(||{
            x.lock().push(2);
            x.lock().push(3);
        });
    });
    let g = x.lock();
    assert!(g.as_slice() == &[1, 2, 3] || g.as_slice() == &[2, 3, 1]);
}
