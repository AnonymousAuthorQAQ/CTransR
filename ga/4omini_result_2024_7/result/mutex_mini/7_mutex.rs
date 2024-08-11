use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(())); // Create a mutex wrapped in an Arc for sharing between threads

    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let loop_count = 2;
        for _ in 0..loop_count {
            let _lock = mutex1.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(_lock); // Explicitly drop the lock
            thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second before starting
        let loop_count = 2;
        for _ in 0..loop_count {
            let _lock = mutex2.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(_lock); // Explicitly drop the lock
            thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
        }
    });

    thread1.join().unwrap(); // Wait for thread1 to finish
    thread2.join().unwrap(); // Wait for thread2 to finish

    println!("The thread is over, process is over too.");
}