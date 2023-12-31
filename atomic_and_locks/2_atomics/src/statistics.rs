use std::time::Duration;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::AtomicU64;
use std::thread;
use std::time::Instant;
use core::sync::atomic::Ordering::Relaxed;
// use std::sync::Arc;
// use std::sync::Mutex;

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);
    // let main_thread = Arc::new(Mutex::new(thread::current()));

    thread::scope(|s| {
        for t in 0..10 {
            // let main_thread = main_thread.clone();
            s.spawn(move || {
                for i in 0..10 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    // let elapsed = start.elapsed().as_millis() as u64;
                    let elapsed = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(elapsed, Relaxed);

                    max_time.fetch_max(elapsed, Relaxed);
                    // main_thread.lock().unwrap().unpark();
                }
            });
        }
        // main_thread.lock().unwrap().unpark();
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 { break; }

            if n == 0 {
                println!("Working 0/100 done");
            } else {
                println!("Working {n}/100 done, total time: {total_time:?}, max time: {max_time:?}");
            }
            // thread::park_timeout(Duration::from_millis(1000));
            thread::sleep(Duration::from_millis(1000));
        }
    });
}

fn process_item(_i: usize) {
    thread::sleep(Duration::from_millis(380));
}
