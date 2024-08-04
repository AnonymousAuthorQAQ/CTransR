use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account: Arc<Mutex<i32>>) {
    let amount = 500000;
    let mut balance = account.lock().unwrap();
    for _ in 0..amount {
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let account1 = Arc::clone(&account_balance);
    let account2 = Arc::clone(&account_balance);

    let handle1 = thread::spawn(move || withdraw(account1));
    let handle2 = thread::spawn(move || withdraw(account2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}