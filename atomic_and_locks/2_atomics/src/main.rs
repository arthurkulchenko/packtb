use std::thread;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        // Will let finish next iteraiton of loop
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }
    STOP.store(true, Relaxed);
    background_thread.join().unwrap();
}

fn some_work() {
    let _ = std::thread::sleep(std::time::Duration::from_secs(1));
    println!("working...");
}
