use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(account_balance: Arc<Mutex<i32>>) {
    for _ in 0..WITHDRAW_AMOUNT {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrement the account balance
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Cloning the Arc to share ownership with threads
    let account_balance_t1 = Arc::clone(&account_balance);
    let account_balance_t2 = Arc::clone(&account_balance);

    // Spawning two threads for withdrawals
    let t1 = thread::spawn(move || {
        withdraw(account_balance_t1);
    });
    
    let t2 = thread::spawn(move || {
        withdraw(account_balance_t2);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}