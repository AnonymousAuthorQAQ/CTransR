use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;
static mut MUTEX: Mutex<()> = Mutex::new(());

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        let _lock = MUTEX.lock().unwrap();
        unsafe {
            ACCOUNT_BALANCE -= 1;
        }
    }
}

fn main() {
    let handle1 = thread::spawn(|| {
        withdraw();
    });

    let handle2 = thread::spawn(|| {
        withdraw();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}