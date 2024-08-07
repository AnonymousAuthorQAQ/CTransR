use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mut handles = Vec::new();
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..2 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut loop_count = 0;
            while loop_count < 2 {
                let mut num = counter.lock().unwrap();
                for _ in 0..10 {
                    println!("Hello world");
                }
                *num += 1;
                thread::sleep(Duration::from_secs(1));
                loop_count += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The threads are over, process is over too.");
}

fn thread1_function(counter: Arc<Mutex<i32>>) {
    let mut loop_count = 0;
    while loop_count < 2 {
        let mut num = counter.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        *num += 1;
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn thread2_function(counter: Arc<Mutex<i32>>) {
    let mut loop_count = 0;
    while loop_count < 2 {
        let mut num = counter.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        *num += 1;
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}