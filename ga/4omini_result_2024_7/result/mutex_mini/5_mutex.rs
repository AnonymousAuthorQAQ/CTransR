use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Shared {
    mutex: Mutex<()>, // Create a Mutex to be shared between threads
}

fn main() {
    let shared = Arc::new(Shared {
        mutex: Mutex::new(()),
    });

    let shared_clone1 = Arc::clone(&shared);
    let thread1 = thread::spawn(move || thread1_function(shared_clone1));

    let shared_clone2 = Arc::clone(&shared);
    let thread2 = thread::spawn(move || thread2_function(shared_clone2));

    thread1.join().expect("Thread 1 panicked");
    thread2.join().expect("Thread 2 panicked");

    println!("The thread is over, process is over too.");
}

fn thread1_function(shared: Arc<Shared>) {
    let loop_count = 2;

    for _ in 0..loop_count {
        let _guard = shared.mutex.lock().expect("Failed to lock mutex");
        for _ in 0..10 {
            println!("Hello world");
        }
        // Mutex guard is dropped here, releasing the lock
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(shared: Arc<Shared>) {
    let loop_count = 2;
    
    // Sleep for 1 second before starting
    thread::sleep(Duration::from_secs(1));

    for _ in 0..loop_count {
        let _guard = shared.mutex.lock().expect("Failed to lock mutex");
        for _ in 0..10 {
            println!("Good morning");
        }
        // Mutex guard is dropped here, releasing the lock
        thread::sleep(Duration::from_secs(1));
    }
}