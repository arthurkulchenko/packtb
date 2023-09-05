use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    with_arc();
    with_channels();
}

pub fn with_arc() {
    let m = String::from("threaded");
    let safe_m = Arc::new(Mutex::new(m));
    let safe_m2 = safe_m.clone();
    let handler = std::thread::spawn(move || {
        safe_m.lock().unwrap().push_str("kakoyipa");
        println!("this new thread with {:?}", safe_m)
    });
    let _ = handler.join();
    println!("general thread with {:?}", safe_m2);
}

fn with_channels() {
                                                      // turbofish notation
    let (sink, stream) = std::sync::mpsc::channel::<Box<dyn Fn(&mut String) + Send>>();
    // let stream = std::sync::Arc::new(std::sync::Mutex::new(stream));
    // let stream2 = stream.clone();

    let handle = std::thread::spawn(move || {
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
    sink.send(Box::new(|s| s.push_str("___Hello"))).unwrap();
    sink.send(Box::new(|s| s.push_str("___World"))).unwrap();
    handle.join().ok();
    drop(sink);
}
