use std::sync::{Arc, Mutex};
use std::thread;
use std::cmp;

fn withdraw(account: Arc<Mutex<i32>>, amount: i32) {
    let mut balance = account.lock().unwrap();
    for _ in 0..amount {
        *balance = cmp::max(0, *balance - 1);
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));
    let withdrawal_amount = 500000;

    let account_balance_clone = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(account_balance_clone, withdrawal_amount);
    });

    let account_balance_clone = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(account_balance_clone, withdrawal_amount);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}