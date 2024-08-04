use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(0i32));

    let mut handles = Vec::new();
    for _ in 0..2 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The thread is over, process is over too.");
}

fn thread1_function() {
    for _ in 0..2 {
        println!("Hello world");
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function() {
    thread::sleep(Duration::from_secs(1));
    for _ in 0..2 {
        println!("Good morning");
        thread::sleep(Duration::from_secs(1));
    }
}