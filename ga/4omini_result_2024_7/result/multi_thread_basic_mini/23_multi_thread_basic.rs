use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Withdraw 1 unit
    }
}

fn main() {
    let initial_balance = 1_000_000;
    let account_balance = Arc::new(Mutex::new(initial_balance));
    let withdraw_amount = 500_000;

    // Clone the Arc (Atomic Reference Counted) to share ownership with threads
    let account_balance1 = Arc::clone(&account_balance);
    let account_balance2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || {
        withdraw(account_balance1, withdraw_amount);
    });

    let t2 = thread::spawn(move || {
        withdraw(account_balance2, withdraw_amount);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}