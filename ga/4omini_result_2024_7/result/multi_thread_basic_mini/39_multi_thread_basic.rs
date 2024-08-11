use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAWAL_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex to safely access the account balance
        let mut cash = balance.lock().unwrap();
        *cash -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Create threads for concurrent withdrawals
    let balance_clone1 = Arc::clone(&account_balance);
    let balance_clone2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || {
        withdraw(balance_clone1, WITHDRAWAL_AMOUNT);
    });

    let t2 = thread::spawn(move || {
        withdraw(balance_clone2, WITHDRAWAL_AMOUNT);
    });

    // Wait for threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}