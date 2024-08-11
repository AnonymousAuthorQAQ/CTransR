use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>) {
    let amount = 500000;
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let account_balance_clone = account_balance.clone();
    let t1 = thread::spawn(move || {
        withdraw(account_balance_clone);
    });

    let account_balance_clone = account_balance.clone();
    let t2 = thread::spawn(move || {
        withdraw(account_balance_clone);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *balance);
}