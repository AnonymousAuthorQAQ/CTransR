use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut handles = Vec::new();

    let counter = Arc::new(Mutex::new(0));

    for _ in 0..2 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("Number: {}", *num);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The program has completed its execution.");
}

fn print_hello() {
    for _ in 0..10 {
        println!("Hello world");
        thread::sleep(Duration::from_secs(1));
    }
}

fn print_good_morning() {
    for _ in 0..10 {
        println!("Good morning");
        thread::sleep(Duration::from_secs(1));
    }
}