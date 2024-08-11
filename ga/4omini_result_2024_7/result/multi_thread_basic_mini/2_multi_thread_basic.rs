use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>) {
    for _ in 0..WITHDRAW_AMOUNT {
        let mut bal = balance.lock().unwrap();
        *bal -= 1;
    }
}

fn main() {
    // Create a shared and thread-safe account balance
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Create two threads to withdraw money
    let account_balance_clone1 = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(account_balance_clone1);
    });

    let account_balance_clone2 = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(account_balance_clone2);
    });

    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}