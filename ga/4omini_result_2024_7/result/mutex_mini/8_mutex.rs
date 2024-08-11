use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    for _ in 0..loop_count {
        // Lock the mutex for mutual exclusion
        let _lock = mutex.lock().unwrap();
        
        for _ in 0..10 {
            println!("Hello world");
        }
        
        // Mutex is automatically unlocked when going out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    thread::sleep(Duration::from_secs(1)); // Sleep before starting

    for _ in 0..loop_count {
        // Lock the mutex for mutual exclusion
        let _lock = mutex.lock().unwrap();
        
        for _ in 0..10 {
            println!("Good morning");
        }
        
        // Mutex is automatically unlocked when going out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    // Create a mutex and wrap it in an Arc for shared ownership between threads
    let mutex = Arc::new(Mutex::new(()));

    // Clone the Arc to pass to threads
    let mutex_clone1 = Arc::clone(&mutex);
    let mutex_clone2 = Arc::clone(&mutex);

    // Create the threads
    let thread1 = thread::spawn(move || thread1_function(mutex_clone1));
    let thread2 = thread::spawn(move || thread2_function(mutex_clone2));

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}