use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex for accessing the shared state
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    // Create an Atomic Reference Counter (Arc) for shared ownership
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Create vector to hold the thread handles
    let mut handles = vec![];

    // Create two threads
    for _ in 0..2 {
        let balance_clone = Arc::clone(&account_balance);
        let handle = thread::spawn(move || {
            withdraw(balance_clone, WITHDRAW_AMOUNT);
        });
        handles.push(handle);
    }

    // Wait for both threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final account balance
    println!("Final account balance: {}", *account_balance.lock().unwrap());
}