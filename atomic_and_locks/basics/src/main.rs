use std::thread;

// Not owned by a thread, cos owned by the program
static X: [i32;3] = [1,2,3];

fn main() {
    let numbers = [1,2,3];

    thread::Builder::new(
        // configure stack size
        // give a name to the thread
    ).spawn(
        || { println!("Builded thread with id: {:?}", thread::current().id()) }
    ).unwrap();

    thread::scope(|s| {
        s.spawn(||{
            println!("lenght: {}", numbers.len());
        });
        s.spawn(||{
            println!("this is {}", thread::current().name().unwrap_or("unnamed"));
            for number in &numbers {
                println!("number: {number}");
            }
        });
    });

    // Will live forever without an owner
    let x: &'static [i32;3] = Box::leak(Box::new([1,2,3]));
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));
}
