use std::thread;
use std::sync::{Arc, Mutex};

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            let mutex = Mutex::new(());
            let _lock = mutex.lock().unwrap();
            ACCOUNT_BALANCE -= 1;
        }
    }
}

fn main() {
    let mutex = Mutex::new(());
    let shared_mutex = Arc::new(mutex);

    let t1_mutex = shared_mutex.clone();
    let t1 = thread::spawn(move || {
        let _lock = t1_mutex.lock().unwrap();
        withdraw();
    });

    let t2_mutex = shared_mutex.clone();
    let t2 = thread::spawn(move || {
        let _lock = t2_mutex.lock().unwrap();
        withdraw();
    });

    t1.join().unwrap();
    t2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}