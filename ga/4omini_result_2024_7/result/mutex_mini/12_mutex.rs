use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;

    while loop_count < 2 {
        let _guard = mutex.lock().unwrap(); // Lock the mutex
        for _ in 0..10 {
            println!("Hello world");
        }
        // Mutex is automatically unlocked when `_guard` goes out of scope
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;
    thread::sleep(Duration::from_secs(1)); // Sleep for 1 second before starting

    while loop_count < 2 {
        let _guard = mutex.lock().unwrap(); // Lock the mutex
        for _ in 0..10 {
            println!("Good morning");
        }
        // Mutex is automatically unlocked when `_guard` goes out of scope
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(())); // Create a mutex wrapped in an Arc

    let mutex_clone1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        thread1_function(mutex_clone1);
    });

    let mutex_clone2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread2_function(mutex_clone2);
    });

    // Wait for the threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("The thread is over, process is over too.");
}