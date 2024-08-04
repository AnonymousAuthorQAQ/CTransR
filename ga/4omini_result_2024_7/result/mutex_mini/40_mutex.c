use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a mutex wrapped in an Arc to share ownership among threads
    let mutex = Arc::new(Mutex::new(()));

    // Clone the mutex reference for thread 1
    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        for _ in 0..2 {
            // Lock the mutex to ensure exclusive access
            let _lock = mutex1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            // Mutex is automatically unlocked when _lock goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Clone the mutex reference for thread 2
    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        // Sleep before starting the second thread to ensure proper order
        thread::sleep(Duration::from_secs(1));
        for _ in 0..2 {
            // Lock the mutex to ensure exclusive access
            let _lock = mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            // Mutex is automatically unlocked when _lock goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}