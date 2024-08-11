use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    for _ in 0..loop_count {
        let _guard = mutex.lock().unwrap(); // Lock the mutex

        for _ in 0..10 {
            println!("Hello world");
        }
    }

    // Sleep for 1 second
    thread::sleep(Duration::from_secs(1));
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    // Sleep for 1 second before starting
    thread::sleep(Duration::from_secs(1));

    for _ in 0..loop_count {
        let _guard = mutex.lock().unwrap(); // Lock the mutex

        for _ in 0..10 {
            println!("Good morning");
        }
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

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The threads are over, process is over too.");
}