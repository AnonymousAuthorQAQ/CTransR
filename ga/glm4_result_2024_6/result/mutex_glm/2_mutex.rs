use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let message1 = Arc::new(Mutex::new("Hello world".to_string()));
    let message2 = Arc::clone(&message1);

    let handle1 = thread::spawn(move || {
        for _ in 0..2 {
            let mut msg = message1.lock().unwrap();
            for _ in 0..10 {
                println!("{}", msg);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..2 {
            let mut msg = message2.lock().unwrap();
            for _ in 0..10 {
                println!("{}", msg);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The threads are over, the process is over too.");
}