use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1000000;
const WITHDRAW_AMOUNT: i32 = 500000;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;  
    }
}

fn main() {
    // Shared account balance wrapped in Arc and Mutex
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Create two threads for withdrawing
    let account_clone1 = Arc::clone(&account_balance);
    let account_clone2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || withdraw(account_clone1, WITHDRAW_AMOUNT));
    let t2 = thread::spawn(move || withdraw(account_clone2, WITHDRAW_AMOUNT));

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}