use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account: Arc<Mutex<i32>>) {
    let mut account = account.lock().unwrap();
    let amount = 500000;
    for _ in 0..amount {
        *account -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));
    
    let account_balance1 = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || withdraw(account_balance1));

    let account_balance2 = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || withdraw(account_balance2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}