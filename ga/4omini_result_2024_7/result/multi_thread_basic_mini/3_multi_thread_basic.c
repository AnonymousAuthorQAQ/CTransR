use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex to safely access the shared account balance
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrement the balance
    }
}

fn main() {
    let initial_balance = 1_000_000;
    let account_balance = Arc::new(Mutex::new(initial_balance)); // Shared, protected balance

    let mut handles = vec![];

    // Create two threads to perform withdrawals
    for _ in 0..2 {
        let account_balance_clone = Arc::clone(&account_balance);
        let handle = thread::spawn(move || {
            let withdrawal_amount = 500_000; // Total amount to withdraw
            withdraw(account_balance_clone, withdrawal_amount);
        });
        handles.push(handle);
    }

    // Wait for both threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Final balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}