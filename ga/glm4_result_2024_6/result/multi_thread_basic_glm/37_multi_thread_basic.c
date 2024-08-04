use std::sync::{Arc, Mutex};
use std::thread;
use std::cmp;

fn withdraw(balance: Arc<Mutex<i32>>, amount: i32) {
    let mut balance_guard = balance.lock().unwrap();
    for _ in 0..amount {
        *balance_guard = cmp::max(0, *balance_guard - 1);
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));
    
    let balance_copy = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(balance_copy, 500000);
    });
    
    let balance_copy = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(balance_copy, 500000);
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.lock().unwrap());
}