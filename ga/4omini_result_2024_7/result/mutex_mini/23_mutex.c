use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    for _ in 0..2 {
        let _guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        drop(_guard); // Explicitly drop the guard here, but it's also done when going out of scope.
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    // Sleep for a second before starting the second thread
    thread::sleep(Duration::from_secs(1));
    for _ in 0..2 {
        let _guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        drop(_guard);
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(())); // Create an Arc with a mutex inside it

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

    println!("The threads are over, process is over too.");
}