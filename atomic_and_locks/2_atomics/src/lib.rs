use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicUsize;
use std::thread;

fn main() {
    // static NUM_DONE: AtomicUsize = AtomicUsize::new(0);
    let num_done: AtomicUsize = AtomicUsize::new(0);
    // let main_thread = std::thread::current();

    thread::scope(|s| {
        s.spawn(|| {
        // let t = thread::spawn(move || {
            for i in 0..10 {
                process(i);
                // num_done.fetch_add(1, Relaxed);
                num_done.store(i + 1, Relaxed);
                // NUM_DONE.store(i + 1, Relaxed);
                // main_thread.unpark();
            }
        });
    // });

    loop {
        // let n = NUM_DONE.load(Relaxed);
        let n = num_done.load(Relaxed);
        if n == 10 { break; };

        println!("working.. {n}/10 done");
        thread::sleep(std::time::Duration::from_secs(1));
        // thread::park_timeout(std::time::Duration::from_secs(1));
    }
    // t.join().unwrap();
    }); // NOTICE: Loop now insize thread scope
    println!("done!");
}

fn process(_index: usize) {
    thread::sleep(std::time::Duration::from_millis(400));
}
