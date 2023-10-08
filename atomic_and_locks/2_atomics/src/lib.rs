// use std::io::Write;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicUsize;
// use std::thread::yield_now;

fn main() {
    static NUM_DONE: AtomicUsize = AtomicUsize::new(0);

    // std::thread::scope(|s| {
        // s.spawn(|| {
        let t = std::thread::spawn(|| {
            for i in 0..100 {
                process(i);
                NUM_DONE.store(i + 1, Relaxed);
            }
        });
    // });

    // println!("before main loop");
    loop {
        // println!("main loop");
        let n = NUM_DONE.load(Relaxed);
        if n == 100 { break; };

        println!("working.. {n}/100 done");
        // std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    t.join().unwrap();
    println!("done!");
}

fn process(_index: usize) {
    std::thread::sleep(std::time::Duration::from_millis(400));
    // yield_now();
}
