use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<i32>>) {
    let mut loop_count = 0;
    while loop_count < 2 {
        let mut guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        drop(guard);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn thread2_function(mutex: Arc<Mutex<i32>>) {
    let mut loop_count = 0;
    thread::sleep(Duration::from_secs(1));
    while loop_count < 2 {
        let mut guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        drop(guard);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(0));

    let mutex1 = Arc::clone(&mutex);
    let handle1 = thread::spawn(move || {
        thread1_function(mutex1);
    });

    let mutex2 = Arc::clone(&mutex);
    let handle2 = thread::spawn(move || {
        thread2_function(mutex2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The thread is over, process is over too.");
}