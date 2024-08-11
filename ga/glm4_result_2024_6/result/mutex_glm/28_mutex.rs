use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));
    let mut handles = Vec::new();

    for _ in 0..2 {
        let mutex = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                println!("Hello world");
            }
            thread::sleep(Duration::from_secs(1));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The threads are over, process is over too.");
}

fn thread1_function() {
    for _ in 0..2 {
        println!("Hello world");
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function() {
    for _ in 0..2 {
        println!("Good morning");
        thread::sleep(Duration::from_secs(1));
    }
}