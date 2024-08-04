use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mut handles = Vec::new();
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..2 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            *count += 1;
            drop(count);
            thread::sleep(Duration::from_secs(1));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The threads are over, process is over too.");
}
