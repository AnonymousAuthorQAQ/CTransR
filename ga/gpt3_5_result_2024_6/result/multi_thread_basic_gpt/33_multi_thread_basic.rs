use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            let mutex = Arc::new(Mutex::new(0));
            let guard = mutex.lock().unwrap();
            ACCOUNT_BALANCE -= 1;
            drop(guard);
        }
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(0));

    let mutex1 = mutex.clone();
    let handle1 = thread::spawn(move || {
        let guard = mutex1.lock().unwrap();
        withdraw();
        drop(guard);
    });

    let mutex2 = mutex.clone();
    let handle2 = thread::spawn(move || {
        let guard = mutex2.lock().unwrap();
        withdraw();
        drop(guard);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}