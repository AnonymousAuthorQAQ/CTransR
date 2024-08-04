use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000; // Initial account balance
const WITHDRAWAL_AMOUNT: i32 = 500_000;  // Amount each thread tries to withdraw

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    let mut balance = account_balance.lock().unwrap();
    *balance -= amount;
}

fn main() {
    // Shared account balance wrapped in Arc and Mutex for thread safety
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));
    
    // Cloning the Arc for each thread
    let account_balance_t1 = Arc::clone(&account_balance);
    let account_balance_t2 = Arc::clone(&account_balance);
    
    let t1 = thread::spawn(move || {
        for _ in 0..WITHDRAWAL_AMOUNT {
            withdraw(account_balance_t1.clone(), 1);
        }
    });

    let t2 = thread::spawn(move || {
        for _ in 0..WITHDRAWAL_AMOUNT {
            withdraw(account_balance_t2.clone(), 1);
        }
    });
    
    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Output the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}