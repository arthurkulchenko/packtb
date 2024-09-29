pub fn call() {
  let mut floats = vec![3.1, 1.2, 4.5, 0.3];
  floats.sort_by(|a, b| a.partial_cmp(b).expect("Error: Uncomparable") );
  println!("{:#?}", floats);
}
