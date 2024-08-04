use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a Mutex wrapped in an Arc to share ownership between threads
    let mutex = Arc::new(Mutex::new(()));

    // Clone the mutex for each thread
    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let loop_count = 2;
        for _ in 0..loop_count {
            // Lock the mutex before printing
            let _guard = mutex1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            // Mutex will be unlocked when _guard goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second before starting
        let loop_count = 2;
        for _ in 0..loop_count {
            // Lock the mutex before printing
            let _guard = mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            // Mutex will be unlocked when _guard goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Print completion message
    println!("The thread is over, process is over too.");
}