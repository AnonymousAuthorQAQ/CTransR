use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;
    for _ in 0..loop_count {
        let _lock = mutex.lock().unwrap(); // Lock the mutex
        for _ in 0..10 {
            println!("Hello world");
        }
        // Mutex gets unlocked when _lock goes out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;
    thread::sleep(Duration::from_secs(1)); // Sleep for 1 second initially
    for _ in 0..loop_count {
        let _lock = mutex.lock().unwrap(); // Lock the mutex
        for _ in 0..10 {
            println!("Good morning");
        }
        // Mutex gets unlocked when _lock goes out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(())); // Create a mutex wrapped in an Arc

    let mutex_clone1 = Arc::clone(&mutex);
    let pthread1 = thread::spawn(move || {
        thread1_function(mutex_clone1);
    });

    let mutex_clone2 = Arc::clone(&mutex);
    let pthread2 = thread::spawn(move || {
        thread2_function(mutex_clone2);
    });

    pthread1.join().unwrap(); // Wait for thread1 to finish
    pthread2.join().unwrap(); // Wait for thread2 to finish
    println!("The thread is over, process is over too.");
}