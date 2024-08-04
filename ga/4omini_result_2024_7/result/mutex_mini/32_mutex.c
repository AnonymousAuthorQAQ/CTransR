use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Use an Arc to allow safe shared ownership between threads
    let mutex = Arc::new(Mutex::new(()));

    let mutex_clone1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let mut loop_count = 0;

        while loop_count < 2 {
            let _lock = mutex_clone1.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Hello world");
            }
            // Mutex is unlocked when _lock goes out of scope
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let mutex_clone2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); // Sleep before starting

        let mut loop_count = 0;
        while loop_count < 2 {
            let _lock = mutex_clone2.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Good morning");
            }
            // Mutex is unlocked when _lock goes out of scope
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("The thread is over, process is over too.");
}