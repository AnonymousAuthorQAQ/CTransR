use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let data1 = Arc::clone(&data);
    let handle1 = thread::spawn(move || {
        let mut loop_count = 0;
        while *data1.lock().unwrap() < 2 {
            for _ in 0..10 {
                println!("Hello world");
            }
            *data1.lock().unwrap() += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    let data2 = Arc::clone(&data);
    let handle2 = thread::spawn(move || {
        let mut loop_count = 0;
        while *data2.lock().unwrap() < 2 {
            for _ in 0..10 {
                println!("Good morning");
            }
            *data2.lock().unwrap() += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The thread is over, process is over too.");
}