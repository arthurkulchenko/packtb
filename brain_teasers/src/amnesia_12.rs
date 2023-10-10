pub fn call() {
  loop {
    let buffer = (0..1000).collect::<Vec<u32>>();
    std::mem::forget(buffer);
    print!(".")
  }
}
