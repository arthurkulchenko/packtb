pub fn call() {
  let c = Box::new([0u32; 10_000_000]);
  println!("{}", c.len());
}
