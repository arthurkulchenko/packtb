// sleepless/cargo.toml
// [package]
// name = "sleepless"
// version = "0.1.0"
// edition = "2018"

// [dependenies]
// tokio = { version = "1.7", features = ["full"] }

// main.rs
use tokio::join;
// use std::time::Duration;
// use chrono::Duration;

async fn count_and_wait(n: u64) -> u64 {
  println!("Starting {}", n);
  // std::thread::sleep(Duration::from_mills(n * 100));
  // std::thread::sleep(Duration::seconds(n));
  std::thread::sleep(std::time::Duration::from_secs(n));
  println!("Returning {}", n);
  n
}

#[tokio::main]
async fn call() -> Result<(), Box<dyn std::error::Error>> {
  join!(count_and_wait(1), count_and_wait(2), count_and_wait(3));
  Ok(())
}
