use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(balance: &Arc<Mutex<i32>>) {
    let amount = 500000;
    for _ in 0..amount {
        if let Ok(mut value) = balance.lock() {
            *value -= 1;
        }
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let balance1 = Arc::clone(&account_balance);
    let t1 = thread::spawn(move || {
        withdraw(&balance1);
    });

    let balance2 = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || {
        withdraw(&balance2);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}