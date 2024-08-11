use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(())); // Using Arc to share the Mutex across threads

    let mutex_clone1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let mut loop_count = 0;

        while loop_count < 2 {
            let _guard = mutex_clone1.lock().unwrap(); // Locking the mutex
            for _ in 0..10 {
                println!("Hello world");
            } // This will run while holding the mutex
            drop(_guard); // Explicitly releasing the lock (optional; it will be released at the end of this scope)

            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let mutex_clone2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        let mut loop_count = 0;
        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second before starting

        while loop_count < 2 {
            let _guard = mutex_clone2.lock().unwrap(); // Locking the mutex
            for _ in 0..10 {
                println!("Good morning");
            } // This will run while holding the mutex
            drop(_guard); // Explicitly releasing the lock

            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The threads are over, process is over too.");
}