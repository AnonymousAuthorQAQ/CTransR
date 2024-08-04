use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&mutex);
    let handle1 = thread::spawn(move || {
        for _ in 0..2 {
            let mut guard = mutex1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(guard);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let handle2 = thread::spawn(move || {
        for _ in 0..2 {
            let mut guard = mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(guard);
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The thread is over, process is over too.");
}