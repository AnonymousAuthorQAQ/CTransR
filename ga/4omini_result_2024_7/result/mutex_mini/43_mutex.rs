use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a mutex
    let mutex = Arc::new(Mutex::new(()));
    
    // Create threads
    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let mut loop_count = 0;
        while loop_count < 2 {
            let _lock = mutex1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        let mut loop_count = 0;
        thread::sleep(Duration::from_secs(1)); // Sleep for the initial time
        while loop_count < 2 {
            let _lock = mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    // Wait for threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("The thread is over, process is over too.");
}