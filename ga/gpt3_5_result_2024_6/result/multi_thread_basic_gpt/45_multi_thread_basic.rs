use std::thread;
use std::sync::{Arc, Mutex};

fn withdraw(account_balance: &Arc<Mutex<i32>>) {
    let mut balance = account_balance.lock().unwrap();
    let mut amount = 500000;
    
    for _ in 0..amount {
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let account_balance1 = Arc::clone(&account_balance);
    let withdraw_handle1 = thread::spawn(move || {
        withdraw(&account_balance1);
    });

    let account_balance2 = Arc::clone(&account_balance);
    let withdraw_handle2 = thread::spawn(move || {
        withdraw(&account_balance2);
    });

    withdraw_handle1.join().unwrap();
    withdraw_handle2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}