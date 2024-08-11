use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    for _ in 0..2 {
        let _lock = mutex.lock().unwrap(); // Acquire the mutex
        for _ in 0..10 {
            println!("Hello world");
        }
        // Mutex is released when `_lock` goes out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    // Sleep for 1 second before starting the second thread
    thread::sleep(Duration::from_secs(1));
    for _ in 0..2 {
        let _lock = mutex.lock().unwrap(); // Acquire the mutex
        for _ in 0..10 {
            println!("Good morning");
        }
        // Mutex is released when `_lock` goes out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(())); // Create a new mutex wrapped in an Arc

    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        thread1_function(mutex1);
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread2_function(mutex2);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}