use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

fn withdraw(account: Arc<Mutex<i32>>) {
    let mut balance = account.lock().unwrap();
    let amount = 500000;
    for _ in 0..amount {
        *balance -= 1;
    }
}

fn main() {
    let account_balance: i32 = 1000000;
    let account = Arc::new(Mutex::new(account_balance));

    let account1 = Arc::clone(&account);
    let handle1 = thread::spawn(move || {
        withdraw(account1);
    });

    let account2 = Arc::clone(&account);
    let handle2 = thread::spawn(move || {
        withdraw(account2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *account.lock().unwrap());
}