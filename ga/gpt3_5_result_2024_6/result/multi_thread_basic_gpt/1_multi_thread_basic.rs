use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    let mutex = Arc::new(Mutex::new(()));

    for _ in 0..amount {
        let _guard = mutex.lock().unwrap();
        unsafe {
            ACCOUNT_BALANCE -= 1;
        }
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&mutex);
    let handle1 = thread::spawn(move || {
        withdraw(mutex1);
    });

    let mutex2 = Arc::clone(&mutex);
    let handle2 = thread::spawn(move || {
        withdraw(mutex2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}