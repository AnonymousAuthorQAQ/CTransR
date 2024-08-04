use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const LOOP_COUNT: usize = 2;

fn main() {
    // Create a mutex protected by an Arc
    let mutex = Arc::new(Mutex::new(()));

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

fn thread1_function(mutex: Arc<Mutex<()>>) {
    for _ in 0..LOOP_COUNT {
        // Lock the mutex
        let _lock = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        drop(_lock); // Explicit drop, though it is dropped at the end of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    // Sleep for a second before starting to simulate the delay
    thread::sleep(Duration::from_secs(1));
    
    for _ in 0..LOOP_COUNT {
        // Lock the mutex
        let _lock = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        drop(_lock); // Explicit drop, though it is dropped at the end of scope
        thread::sleep(Duration::from_secs(1));
    }
}