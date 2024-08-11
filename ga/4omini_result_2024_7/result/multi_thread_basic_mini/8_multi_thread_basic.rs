use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    // Initialize the shared account balance and wrap it in an Arc and Mutex
    let account_balance = Arc::new(Mutex::new(1_000_000));

    // Create two threads to perform the withdrawals
    let account_balance_clone1 = Arc::clone(&account_balance);
    let thread1 = thread::spawn(move || {
        withdraw(account_balance_clone1, 500_000);
    });

    let account_balance_clone2 = Arc::clone(&account_balance);
    let thread2 = thread::spawn(move || {
        withdraw(account_balance_clone2, 500_000);
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}