use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrement the account balance by 1
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1_000_000)); // Initial balance of 1,000,000
    let amount = 500_000;

    let account_balance_clone1 = Arc::clone(&account_balance);
    let thread1 = thread::spawn(move || {
        withdraw(account_balance_clone1, amount);
    });

    let account_balance_clone2 = Arc::clone(&account_balance);
    let thread2 = thread::spawn(move || {
        withdraw(account_balance_clone2, amount);
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Print the final account balance
    println!("Final account balance: {}", *account_balance.lock().unwrap());
}