use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mut threads = Vec::new();
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..2 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut loop_count = 0;
            while loop_count < 2 {
                let mut counter = counter.lock().unwrap();
                for _ in 0..10 {
                    println!("Hello world");
                }
                drop(counter);
                thread::sleep(Duration::from_secs(1));
                loop_count += 1;
            }
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("The threads are over, process is over too.");
}

fn thread1_function(counter: Arc<Mutex<i32>>) {
    let mut loop_count = 0;
    while loop_count < 2 {
        let mut counter = counter.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        drop(counter);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn thread2_function(counter: Arc<Mutex<i32>>) {
    let mut loop_count = 0;
    while loop_count < 2 {
        let mut counter = counter.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        drop(counter);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}