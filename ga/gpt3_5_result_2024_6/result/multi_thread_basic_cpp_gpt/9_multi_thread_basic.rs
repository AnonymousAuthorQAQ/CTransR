use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: &Arc<Mutex<i32>>) {
    let mut balance = account_balance.lock().unwrap();
    for _ in 0..500000 {
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let account_balance_clone = Arc::clone(&account_balance);
    let t1 = thread::spawn(move || {
        withdraw(&account_balance_clone);
    });

    let account_balance_clone = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || {
        withdraw(&account_balance_clone);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}