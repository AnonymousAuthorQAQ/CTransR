use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex_clone1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let mut loop_count = 0;

        while loop_count < 2 {
            let _lock = mutex_clone1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(_lock); // Explicitly drop the lock
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let mutex_clone2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        let mut loop_count = 0;
        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second before starting

        while loop_count < 2 {
            let _lock = mutex_clone2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(_lock); // Explicitly drop the lock
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("The thread is over, process is over too.");
}