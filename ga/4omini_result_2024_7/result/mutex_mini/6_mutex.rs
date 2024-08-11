use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a Mutex wrapped in Arc to allow sharing between threads
    let mutex = Arc::new(Mutex::new(()));

    let mutex_clone1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        for _ in 0..2 {
            // Lock the mutex before executing the critical section
            let _lock = mutex_clone1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            // The lock will be released when _lock goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mutex_clone2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        // Sleep for one second before starting to allow thread1 to run first
        thread::sleep(Duration::from_secs(1));
        for _ in 0..2 {
            // Lock the mutex before executing the critical section
            let _lock = mutex_clone2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            // The lock will be released when _lock goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to complete
    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("The thread is over, process is over too.");
}