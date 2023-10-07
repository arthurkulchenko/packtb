use std::thread;
use std::collections::VecDeque;

// Not owned by a thread, cos owned by the program
static X: [i32;3] = [1,2,3];

#[derive(Debug)]
struct NotSend {
    data: String,
}

fn main() {
    let numbers = [1,2,3];

    thread::Builder::new(
        // configure stack size
        // give a name to the thread
    ).spawn(
        || { println!("Builded thread with id: {:?}", thread::current().id()) }
    ).unwrap();

    let a = std::sync::Arc::new(NotSend { data: "hello".to_string() }.data);

    thread::scope(|s| {
        s.spawn({
            // Arc shadowing the original numbers because each Arc is a separate object.
            let a = a.clone();
            move || {
                println!("lenght: {}", numbers.len());
                println!("{:?}", a);
            }
        });
        s.spawn(||{
            println!("this is {}", thread::current().name().unwrap_or("unnamed"));
            for number in &numbers {
                println!("number: {number}");
            }
        });
    });

    // Will live forever till the end of the programm without an owner, so it could be borrowed by any thread
    let x: &'static [i32;3] = Box::leak(Box::new([1,2,3]));
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));

    // Thread parking ========================================
    let queue = std::sync::Mutex::new(VecDeque::<NotSend>::new());

    thread::scope(|s| {
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        for i in 0.. {
            // vecdeq behind the mutex with not send data type (means that it can't be send on its own)
            queue.lock().unwrap().push_back(NotSend { data: i.to_string() });
            t.thread().unpark();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // Condvar ========================================
    // let queue = std::sync::Mutex::new(VecDeque::<NotSend>::new());
    let not_empty = std::sync::Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        // 'wait' does two things atomically:
                        // It releases the mutex (in this case, the lock on queue).
                        // It blocks the current thread.
                        q = not_empty.wait(q).unwrap();
                    };
                };

                // Another example of Condvar usage:
                // loop {
                //     let mut lock = some_mutex.lock().unwrap();
                //     while !some_condition(&lock) {
                //         lock = some_condvar.wait(lock).unwrap();
                //     }
                //     // Proceed with the logic when the condition is true
                //     // ...
                // }

                // It's essential to wrap the condition check and wait inside a loop
                // because Condvar can experience spurious wakeups (it can wake up even if
                // no thread explicitly notifies it). By checking the condition in a loop,
                // you ensure that the thread only proceeds when the condition is genuinely satisfied.

                drop(q);
                dbg!(item);
            };
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(NotSend { data: i.to_string() });
            not_empty.notify_one();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
