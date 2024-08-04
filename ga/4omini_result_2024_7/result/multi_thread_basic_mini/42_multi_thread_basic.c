use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance wrapped in Arc and Mutex
const INITIAL_BALANCE: i32 = 1_000_000;
static WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex to safely access the shared account balance
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Withdraw $1
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    let balance_clone1 = Arc::clone(&account_balance);
    let balance_clone2 = Arc::clone(&account_balance);

    // Create two threads to perform withdrawals
    let t1 = thread::spawn(move || withdraw(balance_clone1, WITHDRAW_AMOUNT));
    let t2 = thread::spawn(move || withdraw(balance_clone2, WITHDRAW_AMOUNT));

    // Wait for the threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Lock the mutex to read the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}