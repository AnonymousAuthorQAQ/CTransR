use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));
    let mutex_clone1 = Arc::clone(&mutex);
    let mutex_clone2 = Arc::clone(&mutex);

    let thread1 = thread::spawn(move || {
        for _ in 0..2 {
            let _lock = mutex_clone1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(_lock); // Lock is dropped explicitly here (optional)
            thread::sleep(Duration::from_secs(1));
        }
    });

    let thread2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second before starting
        for _ in 0..2 {
            let _lock = mutex_clone2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(_lock); // Lock is dropped explicitly here (optional)
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("The thread is over, process is over too.");
}