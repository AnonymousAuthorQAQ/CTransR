use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a mutex wrapped in an Arc (Atomic Reference Counting)
    let mutex = Arc::new(Mutex::new(()));

    // Clone the mutex to be used in thread1
    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        for _ in 0..2 {
            let _lock = mutex1.lock().unwrap(); // Acquire the mutex lock
            for _ in 0..10 {
                println!("Hello world");
            }
            // Mutex lock is released when `_lock` goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Clone the mutex to be used in thread2
    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); // Sleep for a second before starting
        for _ in 0..2 {
            let _lock = mutex2.lock().unwrap(); // Acquire the mutex lock
            for _ in 0..10 {
                println!("Good morning");
            }
            // Mutex lock is released when `_lock` goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}