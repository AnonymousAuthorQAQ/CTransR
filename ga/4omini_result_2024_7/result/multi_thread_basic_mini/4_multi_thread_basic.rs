use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    // Initial account balance
    let account_balance = Arc::new(Mutex::new(1_000_000));

    // Amount to withdraw
    let withdraw_amount = 500_000;

    // Clone the Arc for each thread
    let account_balance1 = Arc::clone(&account_balance);
    let account_balance2 = Arc::clone(&account_balance);

    // Create threads
    let t1 = thread::spawn(move || {
        withdraw(account_balance1, withdraw_amount);
    });

    let t2 = thread::spawn(move || {
        withdraw(account_balance2, withdraw_amount);
    });

    // Wait for threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}