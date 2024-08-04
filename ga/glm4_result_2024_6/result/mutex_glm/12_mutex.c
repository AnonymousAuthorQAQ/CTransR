use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let counter_cloned = Arc::clone(&counter);

    let handle1 = thread::spawn(move || {
        for _ in 0..2 {
            let mut counter = counter_cloned.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(counter);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..2 {
            let mut counter = counter_cloned.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(counter);
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The threads are over, process is over too.");
}

fn thread1_function() {
    // Defined as an empty function, as Rust handles the logic within the thread closure
}

fn thread2_function() {
    // Defined as an empty function, as Rust handles the logic within the thread closure
}