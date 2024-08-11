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
    unsafe {
        let t1 = thread::spawn(|| {
            withdraw();
        });

        let t2 = thread::spawn(|| {
            withdraw();
        });

        t1.join().unwrap();
        t2.join().unwrap();

        println!("Final account balance: {}", ACCOUNT_BALANCE);

        // Mutex is automatically dropped when goes out of scope
    }
}