use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            let account_balance = &mut ACCOUNT_BALANCE;
            *account_balance -= 1;
        }
    }
}

fn main() {
    let balance_mutex = Arc::new(Mutex::new(()));

    let balance_mutex1 = balance_mutex.clone();
    let t1 = thread::spawn(move || {
        let _lock = balance_mutex1.lock().unwrap();
        withdraw();
    });

    let balance_mutex2 = balance_mutex.clone();
    let t2 = thread::spawn(move || {
        let _lock = balance_mutex2.lock().unwrap();
        withdraw();
    });

    t1.join().unwrap();
    t2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}