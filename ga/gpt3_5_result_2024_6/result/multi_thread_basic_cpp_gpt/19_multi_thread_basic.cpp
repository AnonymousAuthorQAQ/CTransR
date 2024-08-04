use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: &Arc<Mutex<i32>>) {
    let amount = 500000;
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let account_balance_clone1 = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(&account_balance_clone1);
    });

    let account_balance_clone2 = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(&account_balance_clone2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}