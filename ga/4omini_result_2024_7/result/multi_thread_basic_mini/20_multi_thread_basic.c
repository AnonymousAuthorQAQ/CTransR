use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1_000_000));

    let amount = 500_000;
    let account_balance_clone1 = Arc::clone(&account_balance);
    let account_balance_clone2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || {
        withdraw(account_balance_clone1, amount);
    });

    let t2 = thread::spawn(move || {
        withdraw(account_balance_clone2, amount);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    // Display final account balance
    println!("Final account balance: {}", *account_balance.lock().unwrap());
}