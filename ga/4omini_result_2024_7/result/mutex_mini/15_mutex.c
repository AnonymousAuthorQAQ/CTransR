use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    // Create a mutex to synchronize access between threads
    let mutex = Arc::new(Mutex::new(())); // Arc to share ownership between threads

    let mutex1 = Arc::clone(&mutex);
    let handle1 = thread::spawn(move || {
        let mut loop_count = 0;
        while loop_count < 2 {
            // Lock the mutex for this critical section
            let _lock = mutex1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            // Mutex is unlocked when _lock goes out of scope
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let handle2 = thread::spawn(move || {
        let mut loop_count = 0;
        // Sleep to allow the first thread to run first
        thread::sleep(Duration::from_secs(1));
        while loop_count < 2 {
            let _lock = mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("The thread is over, process is over too.");
}