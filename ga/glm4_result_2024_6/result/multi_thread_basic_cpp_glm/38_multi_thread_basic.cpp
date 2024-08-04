use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};

fn withdraw(account: Arc<Mutex<i32>>) {
    let mut balance = account.lock().unwrap();
    let amount = 500000;
    *balance = balance.saturating_sub(amount);
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));
    let account_balance_clone = Arc::clone(&account_balance);

    let handle1 = thread::spawn(move || {
        withdraw(account_balance_clone);
    });

    let handle2 = thread::spawn(move || {
        withdraw(account_balance);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}