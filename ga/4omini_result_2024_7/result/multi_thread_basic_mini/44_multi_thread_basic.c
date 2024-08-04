use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>) {
    for _ in 0..WITHDRAW_AMOUNT {
        let mut account_balance = balance.lock().unwrap();
        *account_balance -= 1; // Withdraw 1 unit
    }
}

fn main() {
    // Wrap the account balance in an Arc (atomic reference counting) and Mutex
    let balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Create two threads to withdraw money concurrently
    let balance_clone1 = Arc::clone(&balance);
    let handle1 = thread::spawn(move || {
        withdraw(balance_clone1);
    });

    let balance_clone2 = Arc::clone(&balance);
    let handle2 = thread::spawn(move || {
        withdraw(balance_clone2);
    });

    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Retrieve the final account balance
    let final_balance = balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}