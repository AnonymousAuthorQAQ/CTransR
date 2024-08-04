use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let message = Arc::new(Mutex::new("".to_string()));

    let message_clone = Arc::clone(&message);
    let handle1 = thread::spawn(move || {
        let mut loop_count = 0;
        while loop_count < 2 {
            let mut messages = message_clone.lock().unwrap();
            for _ in 0..10 {
                messages.push_str("Hello world\n");
            }
            drop(messages);
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let message_clone = Arc::clone(&message);
    let handle2 = thread::spawn(move || {
        let mut loop_count = 0;
        while loop_count < 2 {
            let mut messages = message_clone.lock().unwrap();
            for _ in 0..10 {
                messages.push_str("Good morning\n");
            }
            drop(messages);
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("The threads are over, the process is over too.");
}
