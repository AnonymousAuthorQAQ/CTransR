use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..2 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut loop_count = 0;
            while *counter.lock().unwrap() < 2 {
                println!("Hello world");
                thread::sleep(Duration::from_secs(1));
                *counter.lock().unwrap() += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The thread is over, process is over too.");
}
