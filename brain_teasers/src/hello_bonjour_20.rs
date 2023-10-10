pub fn call() {
  let hello =|| println!("Hello World");
  let hello =|| println!("Bonjour le monde");
  hello();
}
