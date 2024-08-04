use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex to safely access the shared balance
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrement the balance by 1
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1_000_000));
    let amount = 500_000; // Each thread will try to withdraw 500,000

    // Create two threads
    let balance_clone1 = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(balance_clone1, amount);
    });

    let balance_clone2 = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(balance_clone2, amount);
    });

    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Print the final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}