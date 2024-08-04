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

    let balance_clone = Arc::clone(&account_balance);
    let t1 = thread::spawn(move || {
        withdraw(&balance_clone);
    });

    let balance_clone = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || {
        withdraw(&balance_clone);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}