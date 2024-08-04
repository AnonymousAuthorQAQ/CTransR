use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        for _ in 0..2 {
            let _lock = mutex1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(_lock); // Explicitly drop the lock
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); // Sleep before starting to ensure thread1 runs first
        for _ in 0..2 {
            let _lock = mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(_lock); // Explicitly drop the lock
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("The thread is over, process is over too.");
}