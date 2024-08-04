use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let shared_mutex = Arc::new(Mutex::new(()));
    let mutex_clone1 = shared_mutex.clone();
    let mutex_clone2 = shared_mutex.clone();

    let thread1 = thread::spawn(move || {
        thread_function("Hello world", mutex_clone1);
    });

    let thread2 = thread::spawn(move || {
        thread_function("Good morning", mutex_clone2);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}

fn thread_function(message: &str, mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;

    while loop_count < 2 {
        let _lock = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("{}", message);
        }
        drop(_lock);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}