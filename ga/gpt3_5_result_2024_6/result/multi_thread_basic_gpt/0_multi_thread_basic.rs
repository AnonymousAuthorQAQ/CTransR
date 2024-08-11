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
    let balance = Arc::new(Mutex::new(ACCOUNT_BALANCE));

    let balance_clone1 = Arc::clone(&balance);
    let handle1 = thread::spawn(move || {
        withdraw();
    });

    let balance_clone2 = Arc::clone(&balance);
    let handle2 = thread::spawn(move || {
        withdraw();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let final_balance = balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);

    // Mutex automatically dropped here
}
