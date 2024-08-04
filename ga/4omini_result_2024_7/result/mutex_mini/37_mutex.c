use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a mutex wrapped in Arc for shared ownership among threads
    let mutex = Arc::new(Mutex::new(()));

    let mutex_clone1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let mutex_guard = mutex_clone1.lock().unwrap();
        for _ in 0..2 {
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(mutex_guard); // Explicitly drop the lock before sleep
            thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
        }
    });

    let mutex_clone2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second to mimic staggered start
        let mutex_guard = mutex_clone2.lock().unwrap();
        for _ in 0..2 {
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(mutex_guard); // Explicitly drop the lock before sleep
            thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
        }
    });

    thread1.join().unwrap(); // Wait for thread 1 to finish
    thread2.join().unwrap(); // Wait for thread 2 to finish

    println!("The thread is over, process is over too.");
}