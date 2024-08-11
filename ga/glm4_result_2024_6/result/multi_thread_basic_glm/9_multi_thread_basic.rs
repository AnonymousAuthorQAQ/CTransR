use std::sync::{Mutex, Arc};
use std::thread;
use std::ptr;

// Global shared variable representing account balance
let account_balance: Arc<Mutex<i32>> = Arc::new(Mutex::new(1000000));

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || withdraw(account_balance, 500000));
    
    let account_balance = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || withdraw(account_balance, 500000));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}