use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut handles = Vec::new();
    let message = Arc::new(Mutex::new(""));

    for _ in 0..2 {
        let message = Arc::clone(&message);
        let handle = thread::spawn(move || {
            let messages = ["Hello world", "Good morning"];
            for _ in 0..2 {
                for msg in &messages {
                    *message.lock().unwrap() = msg;
                    println!("{}", msg);
                }
                thread::sleep(Duration::from_secs(1));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The thread is over, process is over too.");
}