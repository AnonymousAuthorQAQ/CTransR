use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(0));

    let mutex_clone1 = mutex.clone();
    let thread1 = thread::spawn(move || thread1_function(mutex_clone1));

    let mutex_clone2 = mutex.clone();
    let thread2 = thread::spawn(move || thread2_function(mutex_clone2));

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}

fn thread1_function(mutex: Arc<Mutex<i32>>) {
    let mut data = mutex.lock().unwrap();

    let mut loop_count = 0;
    while loop_count < 2 {
        for _ in 0..10 {
            println!("Hello world");
        }
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn thread2_function(mutex: Arc<Mutex<i32>>) {
    let mut data = mutex.lock().unwrap();

    thread::sleep(Duration::from_secs(1));

    let mut loop_count = 0;
    while loop_count < 2 {
        for _ in 0..10 {
            println!("Good morning");
        }
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}