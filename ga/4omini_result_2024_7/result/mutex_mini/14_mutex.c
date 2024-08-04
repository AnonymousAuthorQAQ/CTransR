use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a mutex for synchronizing access
    let mutex = Arc::new(Mutex::new(()));

    // Create cloned arc for each thread
    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        for _ in 0..2 {
            // Lock the mutex
            let _lock = mutex1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            // Mutex guard is dropped here when going out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mutex2 = Arc::clone(&mutex);
    // Spawn the second thread
    let thread2 = thread::spawn(move || {
        // Sleep to stagger the start of this thread
        thread::sleep(Duration::from_secs(1));
        for _ in 0..2 {
            // Lock the mutex
            let _lock = mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            // Mutex guard is dropped here when going out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The threads are over, process is over too.");
}