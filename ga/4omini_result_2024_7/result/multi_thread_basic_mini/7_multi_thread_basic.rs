use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAWAL_AMOUNT: i32 = 500_000;

fn withdraw(account_balance: Arc<Mutex<i32>>) {
    for _ in 0..WITHDRAWAL_AMOUNT {
        let mut balance = account_balance.lock().unwrap(); // Lock the mutex
        *balance -= 1; // Withdraw 1 unit
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE)); // Use Arc for shared ownership
    
    let balance_clone1 = Arc::clone(&account_balance);
    let balance_clone2 = Arc::clone(&account_balance);

    // Create two threads
    let t1 = thread::spawn(move || {
        withdraw(balance_clone1);
    });

    let t2 = thread::spawn(move || {
        withdraw(balance_clone2);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}