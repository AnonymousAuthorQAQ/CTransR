use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    for _ in 0..loop_count {
        let _guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        drop(_guard); // Explicitly drop guard to unlock the mutex
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;
    thread::sleep(Duration::from_secs(1)); // Sleep for 1 second before starting the thread

    for _ in 0..loop_count {
        let _guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        drop(_guard); // Explicitly drop guard to unlock the mutex
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(())); // Create an Arc-wrapped Mutex

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