// waiting_for_godot/Cargo.toml
// [package]
// name = "waiting_for_godot"
// version = "0.1.0"
// edition = "2018"

// [dependencies]
// tokio = { version = "1", features = ["full"] }

// waiting_for_godot/Cargo.toml
async fn hello() {
  println!("Hello, World!")
}

#[tokio::main]
pub async fn call() -> Result<(), Box<dyn std::error::Error>> {
  hello();
  Ok(())
}
