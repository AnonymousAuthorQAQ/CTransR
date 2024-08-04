use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a Mutex wrapped in an Arc for shared ownership across threads
    let mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        for _ in 0..2 {
            let _guard = mutex1.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Hello world");
            }
            // Mutex is automatically unlocked when _guard goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); // Sleep before starting thread2
        for _ in 0..2 {
            let _guard = mutex2.lock().unwrap(); // Lock the mutex
            for _ in 0..10 {
                println!("Good morning");
            }
            // Mutex is automatically unlocked when _guard goes out of scope
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("The thread is over, process is over too.");
}