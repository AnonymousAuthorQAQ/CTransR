use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));  // Shared account balance with a mutex
    let mut handles = Vec::new();  // Vector to hold thread handles

    for _ in 0..2 {
        let account_balance = Arc::clone(&account_balance);  // Clone the shared account balance
        let handle = thread::spawn(move || {
            let mut balance = account_balance.lock().unwrap();  // Lock the mutex and get the balance
            for _ in 0..500000 {
                *balance -= 1;  // Decrement the balance
            }
        });
        handles.push(handle);  // Push the handle to the vector
    }

    for handle in handles {
        handle.join().unwrap();  // Wait for all threads to finish
    }

    println!("Final account balance: {}", *account_balance.lock().unwrap());  // Print the final balance

    // No need to explicitly destroy the mutex or Arc, they are automatically cleaned up when they go out of scope
}