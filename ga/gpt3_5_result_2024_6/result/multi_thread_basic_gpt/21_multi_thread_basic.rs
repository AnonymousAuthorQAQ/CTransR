use std::thread;
use std::sync::{Mutex, Arc};

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            ACCOUNT_BALANCE -= 1;
        }
    }
}

fn main() {
    let balance_mutex = Arc::new(Mutex::new(()));

    let balance_mutex1 = Arc::clone(&balance_mutex);
    let handle1 = thread::spawn(move || {
        let _lock = balance_mutex1.lock().unwrap();
        withdraw();
    });

    let balance_mutex2 = Arc::clone(&balance_mutex);
    let handle2 = thread::spawn(move || {
        let _lock = balance_mutex2.lock().unwrap();
        withdraw();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}