use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAWAL_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>, amount: i32) {
    // Each thread will withdraw the specified amount
    for _ in 0..amount {
        // Lock the mutex before modifying the balance
        let mut balance_guard = balance.lock().unwrap();
        *balance_guard -= 1; // Decrement the account balance by 1
    }
}

fn main() {
    let balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    let mut handles = vec![];

    // Create two threads to withdraw money concurrently
    for _ in 0..2 {
        let balance_clone = Arc::clone(&balance);
        let handle = thread::spawn(move || {
            withdraw(balance_clone, WITHDRAWAL_AMOUNT);
        });
        handles.push(handle);
    }

    // Wait for both threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Access the final balance
    let final_balance = *balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}