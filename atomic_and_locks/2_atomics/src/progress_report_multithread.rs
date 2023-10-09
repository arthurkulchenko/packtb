use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
// use std::thread::JoinHandle;


fn main() {
    let cpus: usize = num_cpus::get_physical();
    static NUM_DONE: AtomicUsize = AtomicUsize::new(0);
    let mut handlers = vec![];

    // std::thread::scope(|s| {
    let amount_to_split = 100;
    let chunk_size = amount_to_split / cpus;

    // split array onto ranges and then make it consumed by threads
    for t in 0..cpus {
        let spawned_thread = std::thread::spawn(move || {
            let start = chunk_size * t;
            let end = start + chunk_size;
            for i in start..end {
                process_item(t * 25 + i);
                NUM_DONE.fetch_add(1, Relaxed);
            }
        });
        handlers.push(spawned_thread);
    }
    // });

    loop {
        let n = NUM_DONE.load(Relaxed);
        if n == 100 { break; }

        println!("Working {n}/100 done");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        // std::thread::park_timeout(std::time::Duration::from_millis(1000));
    }
    handlers.into_iter().for_each(|h| h.join().unwrap());
    // std::thread::current().unpark();

    println!("Done!");
}

fn process_item(_i: usize) {
    std::thread::sleep(std::time::Duration::from_millis(400));
}
