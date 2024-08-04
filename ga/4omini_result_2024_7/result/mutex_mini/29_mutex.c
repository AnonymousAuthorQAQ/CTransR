use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a mutex wrapped in an Arc for shared ownership across threads
    let mutex = Arc::new(Mutex::new(()));

    // Create threads
    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        thread1_function(mutex1);
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread2_function(mutex2);
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;

    while loop_count < 2 {
        // Lock the mutex
        let _guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        // Mutex is automatically unlocked when _guard goes out of scope
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;

    // Sleep for 1 second before starting
    thread::sleep(Duration::from_secs(1));

    while loop_count < 2 {
        // Lock the mutex
        let _guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        // Mutex is automatically unlocked when _guard goes out of scope
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}