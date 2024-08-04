use std::sync::{Arc, Mutex};
use std::thread;

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
    let mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&mutex);
    let t1 = thread::spawn(move || {
        let _guard = mutex1.lock().unwrap();
        withdraw();
    });

    let mutex2 = Arc::clone(&mutex);
    let t2 = thread::spawn(move || {
        let _guard = mutex2.lock().unwrap();
        withdraw();
    });
    
    t1.join().unwrap();
    t2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}
