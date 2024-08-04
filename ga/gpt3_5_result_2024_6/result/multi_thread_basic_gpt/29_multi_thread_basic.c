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
    let account_balance = Arc::new(Mutex::new(ACCOUNT_BALANCE));

    let account_balance1 = Arc::clone(&account_balance);
    let account_balance2 = Arc::clone(&account_balance);

    let handle1 = thread::spawn(move || {
        withdraw();
    });

    let handle2 = thread::spawn(move || {
        withdraw();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.lock().unwrap());

    // Mutex is automatically dropped
}