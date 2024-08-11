use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
const INITIAL_BALANCE: i32 = 1_000_000;

// Withdraw function
fn withdraw(balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut bal = balance.lock().unwrap();
        *bal -= 1; // Withdraw 1 unit
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));
    
    // Amount to withdraw
    let withdraw_amount = 500_000;
    
    // Create threads for withdrawal
    let balance1 = Arc::clone(&account_balance);
    let thread1 = thread::spawn(move || {
        withdraw(balance1, withdraw_amount);
    });
    
    let balance2 = Arc::clone(&account_balance);
    let thread2 = thread::spawn(move || {
        withdraw(balance2, withdraw_amount);
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Print the final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}