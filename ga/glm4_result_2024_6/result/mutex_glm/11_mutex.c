use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut handles = Vec::new();

    let print_mutex = Arc::new(Mutex::new(()));

    for _ in 0..2 {
        let print_mutex = Arc::clone(&print_mutex);
        let handle = thread::spawn(move || {
            for _ in 0..2 {
                let mut lock = print_mutex.lock().unwrap();
                for _ in 0..10 {
                    println!("Hello world");
                }
                drop(lock);
                thread::sleep(Duration::from_secs(1));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The threads are over, process is over too.");
}

fn thread1_function(print_mutex: Arc<Mutex<()>>) {
    for _ in 0..2 {
        let mut lock = print_mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        drop(lock);
        thread::sleep(Duration::from_secs(1));
    }
}