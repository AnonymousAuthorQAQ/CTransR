use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            let mutex = Arc::new(Mutex::new(()));
            let _lock = mutex.lock().unwrap();
            ACCOUNT_BALANCE -= 1;
        }
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let t1_mutex = Arc::clone(&mutex);
    let t1 = thread::spawn(move || {
        withdraw();
    });

    let t2_mutex = Arc::clone(&mutex);
    let t2 = thread::spawn(move || {
        withdraw();
    });

    t1.join().unwrap();
    t2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}