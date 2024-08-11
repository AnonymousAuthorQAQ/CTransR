use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    let mut handles = Vec::new();

    let data = Arc::new(Mutex::new(()));

    for _ in 0..2 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut loop_count = 0;
            while loop_count < 2 {
                let mut data = data.lock().unwrap();
                for _ in 0..10 {
                    println!("Hello world");
                }
                drop(data);
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
