use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicUsize;

fn main() {
    static NUM_DONE: AtomicUsize = AtomicUsize::new(0);
    let main_thread = std::thread::current();

    // std::thread::scope(|s| {
        // s.spawn(|| {
        let t = std::thread::spawn(move || {
            for i in 0..10 {
                process(i);
                NUM_DONE.store(i + 1, Relaxed);
                main_thread.unpark();
            }
        });
    // });

    loop {
        let n = NUM_DONE.load(Relaxed);
        if n == 10 { break; };

        println!("working.. {n}/10 done");
        // std::thread::sleep(std::time::Duration::from_secs(1));
        std::thread::park_timeout(std::time::Duration::from_secs(1));
    }
    t.join().unwrap();
    println!("done!");
}

fn process(_index: usize) {
    std::thread::sleep(std::time::Duration::from_millis(400));
}
