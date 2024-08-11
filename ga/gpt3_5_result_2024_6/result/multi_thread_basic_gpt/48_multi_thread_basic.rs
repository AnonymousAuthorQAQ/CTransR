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
    // Creating a Mutex to protect the account balance
    let account_balance = Arc::new(Mutex::new(ACCOUNT_BALANCE));

    let account_balance_clone = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        let mut balance = account_balance_clone.lock().unwrap();
        withdraw();
        *balance -= 1;
    });

    let account_balance_clone = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        let mut balance = account_balance_clone.lock().unwrap();
        withdraw();
        *balance -= 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", unsafe { ACCOUNT_BALANCE });

    // Mutex is automatically dropped when it goes out of scope
}