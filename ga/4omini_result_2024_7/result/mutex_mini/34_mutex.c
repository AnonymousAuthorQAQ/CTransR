use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    for _ in 0..loop_count {
        // Lock the mutex
        let _lock = mutex.lock().unwrap();

        for _ in 0..10 {
            println!("Hello world");
        }
        
        // Release the mutex by going out of scope 
        drop(_lock);

        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    // Sleep for a bit before starting the output
    thread::sleep(Duration::from_secs(1));

    for _ in 0..loop_count {
        // Lock the mutex
        let _lock = mutex.lock().unwrap();

        for _ in 0..10 {
            println!("Good morning");
        }

        // Release the mutex by going out of scope 
        drop(_lock);
        
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let mutex: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    let mutex_clone1 = Arc::clone(&mutex);
    let mutex_clone2 = Arc::clone(&mutex);

    let thread1 = thread::spawn(move || {
        thread1_function(mutex_clone1);
    });

    let thread2 = thread::spawn(move || {
        thread2_function(mutex_clone2);
    });

    // Wait for the threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The threads are over, process is over too.");
}