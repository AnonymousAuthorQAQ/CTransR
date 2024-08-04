use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;
    
    while loop_count < 2 {
        let _lock = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        drop(_lock);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;
    thread::sleep(Duration::from_secs(1));
    
    while loop_count < 2 {
        let _lock = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        drop(_lock);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex_clone = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        thread1_function(mutex_clone);
    });

    let mutex_clone = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread2_function(mutex_clone);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}