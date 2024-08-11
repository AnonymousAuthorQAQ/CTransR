use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;

    for _ in 0..loop_count {
        let _lock = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        // Mutex is automatically released when going out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let loop_count = 2;
    thread::sleep(Duration::from_secs(1)); // Sleep initially
    
    for _ in 0..loop_count {
        let _lock = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        // Mutex is automatically released when going out of scope
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex_clone = Arc::clone(&mutex);
    let handle1 = thread::spawn(move || {
        thread1_function(mutex_clone);
    });

    let mutex_clone = Arc::clone(&mutex);
    let handle2 = thread::spawn(move || {
        thread2_function(mutex_clone);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("The thread is over, process is over too.");
}