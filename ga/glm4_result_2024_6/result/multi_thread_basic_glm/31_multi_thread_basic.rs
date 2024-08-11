use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    // Initialize the shared account balance
    let account_balance = Arc::new(Mutex::new(1000000));

    // Create two threads that execute the withdraw function
    let handles = (0..2).map(|_| {
        let account_balance = Arc::clone(&account_balance);
        thread::spawn(move || {
            let mut balance = account_balance.lock().unwrap();
            for _ in 0..500000 {
                *balance -= 1;
            }
        })
    });

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final account balance
    println!("Final account balance: {}", *account_balance.lock().unwrap());
}