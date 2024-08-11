use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let shared_mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&shared_mutex);
    let thread1 = thread::spawn(move || {
        let mut loop_count = 0;
        while loop_count < 2 {
            let _lock = mutex1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(_lock);
            thread::sleep(Duration::from_secs(1));

            loop_count += 1;
        }
    });

    let mutex2 = Arc::clone(&shared_mutex);
    let thread2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let mut loop_count = 0;
        while loop_count < 2 {
            let _lock = mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(_lock);
            thread::sleep(Duration::from_secs(1));

            loop_count += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}