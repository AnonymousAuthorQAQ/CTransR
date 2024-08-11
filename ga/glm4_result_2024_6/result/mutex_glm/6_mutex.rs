use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let handle1 = Arc::new(Mutex::new(0));
    let handle2 = Arc::clone(&handle1);

    let handle1_thread = thread::spawn(move || {
        for _ in 0..2 {
            let mut lock = handle1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(lock);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2_thread = thread::spawn(move || {
        for _ in 0..2 {
            let mut lock = handle2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(lock);
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle1_thread.join().unwrap();
    handle2_thread.join().unwrap();
    println!("The thread is over, process is over too.");
}
