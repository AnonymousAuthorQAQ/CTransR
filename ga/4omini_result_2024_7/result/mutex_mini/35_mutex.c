use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex_clone1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let loop_count = 2;
        for _ in 0..loop_count {
            let _lock = mutex_clone1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            // Mutex is automatically unlocked when it goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mutex_clone2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        let loop_count = 2;
        thread::sleep(Duration::from_secs(1)); // Wait for a bit before starting
        for _ in 0..loop_count {
            let _lock = mutex_clone2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            // Mutex is automatically unlocked when it goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The threads are over, process is over too.");
}