use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

fn withdraw(account: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));
    let amount = 500000;

    let account_balance1 = Arc::clone(&account_balance);
    let account_balance2 = Arc::clone(&account_balance);

    let handle1 = thread::spawn(move || {
        withdraw(account_balance1, amount);
    });

    let handle2 = thread::spawn(move || {
        withdraw(account_balance2, amount);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}