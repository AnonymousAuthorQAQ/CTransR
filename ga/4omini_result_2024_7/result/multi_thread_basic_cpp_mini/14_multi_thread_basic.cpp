use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let initial_balance = 1_000_000;
    let account_balance = Arc::new(Mutex::new(initial_balance));

    // Define the amount to withdraw
    let withdraw_amount = 500_000;

    // Create threads
    let account_balance_clone = Arc::clone(&account_balance);
    let t1 = thread::spawn(move || {
        withdraw(account_balance_clone, withdraw_amount);
    });

    let account_balance_clone = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || {
        withdraw(account_balance_clone, withdraw_amount);
    });

    // Wait for threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}