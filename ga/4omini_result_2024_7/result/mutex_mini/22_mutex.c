use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(())); // Create a mutex wrapped in an Arc for shared ownership

    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let mut loop_count = 0;

        while loop_count < 2 {
            let _lock = mutex1.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Hello world");
            }
            // Mutex is automatically unlocked when `_lock` goes out of scope
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        let mut loop_count = 0;
        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second before starting

        while loop_count < 2 {
            let _lock = mutex2.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Good morning");
            }
            // Mutex is automatically unlocked when `_lock` goes out of scope
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    thread1.join().unwrap(); // Wait for thread 1 to finish
    thread2.join().unwrap(); // Wait for thread 2 to finish
    println!("The thread is over, process is over too.");
}