use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut handles = Vec::new();
    let message_mutex = Arc::new(Mutex::new(""));

    for _ in 0..2 {
        let message_mutex = Arc::clone(&message_mutex);
        let handle = thread::spawn(move || {
            let mut loop_count = 0;
            while loop_count < 2 {
                let mut message = message_mutex.lock().unwrap();
                for _ in 0..10 {
                    println!("Hello world");
                }
                drop(message);
                thread::sleep(Duration::from_secs(1));
                loop_count += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The thread is over, process is over too.");
}

fn thread1_function() {
    // Function body is the same as in the main function
}

fn thread2_function() {
    // Function body is the same as in the main function
}