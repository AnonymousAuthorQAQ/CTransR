use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a Mutex wrapped in an Arc (Atomic Reference Counted)
    let mutex = Arc::new(Mutex::new(()));

    // Clone the Arc for use in thread1
    let mutex_clone1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let mut loop_count = 0;

        while loop_count < 2 {
            let _lock = mutex_clone1.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Hello world");
            }
            // No need to unlock manually; it unlocks when `_lock` goes out of scope
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    // Clone the Arc for use in thread2
    let mutex_clone2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        let mut loop_count = 0;

        thread::sleep(Duration::from_secs(1)); // Sleep before executing
        while loop_count < 2 {
            let _lock = mutex_clone2.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Good morning");
            }
            // No need to unlock manually; it unlocks when `_lock` goes out of scope
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}