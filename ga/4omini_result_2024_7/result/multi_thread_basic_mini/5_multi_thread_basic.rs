use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    let mut balance = account_balance.lock().unwrap();
    *balance -= amount; // Decrement the account balance directly.
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Create threads
    let thread1_balance = Arc::clone(&account_balance);
    let thread2_balance = Arc::clone(&account_balance);
    
    let t1 = thread::spawn(move || {
        withdraw(thread1_balance, WITHDRAW_AMOUNT);
    });

    let t2 = thread::spawn(move || {
        withdraw(thread2_balance, WITHDRAW_AMOUNT);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}