use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>) {
    let mut balance = account_balance.lock().unwrap();
    *balance -= 500000;
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let account_balance_clone1 = Arc::clone(&account_balance);
    let t1 = thread::spawn(move || {
        withdraw(account_balance_clone1);
    });

    let account_balance_clone2 = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || {
        withdraw(account_balance_clone2);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}