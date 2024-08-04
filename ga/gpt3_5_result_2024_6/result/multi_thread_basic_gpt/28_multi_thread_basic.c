use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            let mutex = Arc::new(Mutex::new(ACCOUNT_BALANCE));
            let shared_mutex = mutex.clone();
            let mut account_balance = shared_mutex.lock().unwrap();
            *account_balance -= 1;
        }
    }
}

fn main() {
    let t1 = thread::spawn(|| {
        withdraw();
    });

    let t2 = thread::spawn(|| {
        withdraw();
    });

    t1.join().unwrap();
    t2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}
