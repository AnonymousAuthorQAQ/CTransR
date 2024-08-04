use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1000000;
const WITHDRAW_AMOUNT: i32 = 500000;

fn withdraw(account_balance: Arc<Mutex<i32>>) {
    // Each thread will "withdraw" 500,000 in a loop
    for _ in 0..WITHDRAW_AMOUNT {
        // Lock the mutex to safely access the shared account balance
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrement the account balance by 1
    }
}

fn main() {
    // Create a Mutex to hold the account balance and wrap it in an Arc for shared ownership
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Create two threads for withdrawal
    let account_balance_clone1 = Arc::clone(&account_balance);
    let account_balance_clone2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || {
        withdraw(account_balance_clone1);
    });

    let t2 = thread::spawn(move || {
        withdraw(account_balance_clone2);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    println!("Final account balance: {}", *account_balance.lock().unwrap());
}