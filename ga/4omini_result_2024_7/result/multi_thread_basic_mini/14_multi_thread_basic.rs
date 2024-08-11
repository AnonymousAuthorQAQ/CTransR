use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000; // Global shared variable representing account balance
const WITHDRAW_AMOUNT: i32 = 500_000;    // Amount to withdraw

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex to access the shared account balance
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrement the balance by 1
    }
}

fn main() {
    // Create an Arc (Atomic Reference Counted) Mutex to share the account balance across threads
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));
    
    // Clone the Arc to pass to the threads
    let account_balance1 = Arc::clone(&account_balance);
    let account_balance2 = Arc::clone(&account_balance);
    
    // Create the withdrawal threads
    let t1 = thread::spawn(move || withdraw(account_balance1, WITHDRAW_AMOUNT));
    let t2 = thread::spawn(move || withdraw(account_balance2, WITHDRAW_AMOUNT));

    // Wait for the threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}