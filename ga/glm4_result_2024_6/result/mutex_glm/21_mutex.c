use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));
    let mut handles = Vec::new();

    for _ in 0..2 {
        let mutex_clone = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            let mut loop_count = 0;
            while loop_count < 2 {
                let _guard = mutex_clone.lock().unwrap();
                for _ in 0..10 {
                    println!("Hello world");
                }
                thread::sleep(Duration::from_secs(1));
                loop_count += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The thread is over, process is over too.");
}