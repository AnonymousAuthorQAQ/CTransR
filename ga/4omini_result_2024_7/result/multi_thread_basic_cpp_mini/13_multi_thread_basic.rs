use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Initialize account balance with 1,000,000
    let account_balance = Arc::new(Mutex::new(1_000_000));
    let amount = 500_000;

    // Create two threads that will withdraw from the account
    let mut handles = vec![];

    for _ in 0..2 {
        let balance_clone = Arc::clone(&account_balance);
        let handle = thread::spawn(move || {
            let mut balance = balance_clone.lock().unwrap();
            for _ in 0..amount {
                *balance -= 1;
            }
        });
        handles.push(handle);
    }

    // Wait for both threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}