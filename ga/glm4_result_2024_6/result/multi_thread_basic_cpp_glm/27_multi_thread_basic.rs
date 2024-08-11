use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>) {
    let amount = 500000;
    let mut balance = account_balance.lock().unwrap();
    *balance -= amount;
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let account_balance_clone = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(account_balance_clone);
    });

    let account_balance_clone = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(account_balance_clone);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.lock().unwrap());
}