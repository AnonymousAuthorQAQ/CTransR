use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    for _ in 0..loop_count {
        let _lock = mutex.lock().unwrap(); // Acquire the lock
        for _ in 0..10 {
            println!("Hello world");
        }
        // Lock is released when _lock goes out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    thread::sleep(Duration::from_secs(1)); // Sleep for a second before starting

    for _ in 0..loop_count {
        let _lock = mutex.lock().unwrap(); // Acquire the lock
        for _ in 0..10 {
            println!("Good morning");
        }
        // Lock is released when _lock goes out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(())); // Create an Arc (atomic reference count) to the mutex

    let mutex_clone1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        thread1_function(mutex_clone1);
    });

    let mutex_clone2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread2_function(mutex_clone2);
    });

    thread1.join().unwrap(); // Wait for thread1 to finish
    thread2.join().unwrap(); // Wait for thread2 to finish

    println!("The thread is over, process is over too.");
}