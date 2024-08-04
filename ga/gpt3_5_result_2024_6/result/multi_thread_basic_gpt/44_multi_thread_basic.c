use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            let mutex = &MUTEX;
            let mut balance = mutex.lock().unwrap();
            *balance -= 1;
        }
    }
}

// Mutex for protecting the account balance
lazy_static::lazy_static! {
    static ref MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(ACCOUNT_BALANCE));
}

fn main() {
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            withdraw();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}