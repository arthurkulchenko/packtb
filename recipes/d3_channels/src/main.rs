use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    with_arc();
    with_channels();
}

pub fn with_arc() {
    let m = String::from("threaded");
    let safe_m = Arc::new(Mutex::new(m));
    let safe_m2 = safe_m.clone();
    let handler = thread::spawn(move || {
        safe_m.lock().unwrap().push_str("kakoyipa");
        println!("this new thread with {:?}", safe_m)
    });
    let _ = handler.join();
    println!("general thread with {:?}", safe_m2);
}

fn with_channels() {
                                //    |------------------ turbofish notation
    let (sink, stream) = mpsc::channel::<Box<dyn Fn(&mut String) + Send>>();
    // let stream = std::sync::Arc::new(std::sync::Mutex::new(stream));
    // let stream2 = stream.clone();

    let handle = thread::spawn(move || {
        let mut hidden = String::new();
        loop {
            match stream.recv() {
                Ok(f) => {
                    f(&mut hidden);
                    println!("hidden: {}", hidden);
                    if hidden == "___Hello___World" {
                        println!("Done");
                        return;
                    }
                },
                Err(_) => {
                    println!("Done");
                    return;
                },
            }
        }
    });
    // Created channel with boxed function that takes string mutable reference
    // mpsc::channel::<Box<dyn Fn(&mut String) + Send>>()
    // so here is clear to us when we send that we have to send a boxed function
    // and we can do whatever we want with the string reference only in this
    // specification Box::new(|s| s...) s represents a state of the channel
    // it is a structure and a state.
    sink.send(Box::new(|s| s.push_str("___Hello"))).unwrap();
    sink.send(Box::new(|s| s.push_str("___World"))).unwrap();
    handle.join().ok();
    drop(sink);
}
