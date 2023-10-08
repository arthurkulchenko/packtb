use std::time::SystemTime;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicU64;

fn main() {
    let timer = SystemTime::now();
    let x = get_x();
    let duration = timer.elapsed().unwrap();
    println!("first invoke takes {duration:?}, x: {}", x);

    let timer = SystemTime::now();
    let x = get_x();
    let duration = timer.elapsed().unwrap();
    println!("second invoke takes: {duration:?}, x: {}", x);
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn calculate_x() -> u64{
    println!("calculating x... once");
    std::thread::sleep(std::time::Duration::from_secs(1));
    42
}
