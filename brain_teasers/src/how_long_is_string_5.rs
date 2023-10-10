const HELLO_WORLD : &'static str = "Hellœ heimur";

pub fn call() {
  println!("{} is {} characters long.", HELLO_WORLD, HELLO_WORLD.len());
}
