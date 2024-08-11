use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::MutexGuard;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            let _lock = MUTEX.lock().unwrap();
            ACCOUNT_BALANCE -= 1;
        }
    }
}

// Mutex for protecting the account balance
lazy_static::lazy_static! {
    static ref MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

fn main() {
    let mutex = Arc::clone(&MUTEX);

    let handle1 = thread::spawn(move || {
        withdraw();
    });

    let handle2 = thread::spawn(move || {
        withdraw();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        let balance: MutexGuard<i32> = MUTEX.lock().unwrap();
        println!("Final account balance: {}", *balance);
    }
}